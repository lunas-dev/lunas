// core.mjs — Lunas reactive core: compile-time adjacency dispatch.
// See crates/lunas_compiler/docs/output-design.md §4–5.
//
// Plain ESM, ES2015+ (Proxy floor is set elsewhere, in boxes.mjs). No BigInt.
//
// A component context `c` is:
//   { root, deps: [], queue: [], pending: false, scope: null }
// where deps[i] is the adjacency list of bind records that read reactive
// variable i. `scope` is the currently-open collection scope used by control
// flow blocks so their inner binds can be unregistered on teardown (see
// for-diff-design.md §6).

export function createContext(root) {
  // `parent` links a child component context to the one that mounted it
  // (additive; set by mountChild) so provide/inject (provide.mjs) can walk the
  // chain. `onUpdate` (lifecycle.mjs) is an optional per-context hook the flush
  // loop invokes after an update pass actually ran. All new fields default to
  // null so contexts stay cheap and existing paths are untouched.
  return {
    root,
    deps: [],
    queue: [],
    pending: false,
    scope: null,
    post: null,
    parent: null,
    onUpdate: null,
    gen: 0, // current flush-burst id (runaway-loop guard; see flush)
  };
}

// Cap on how many times a SINGLE effect may re-run within one flush burst. A
// reactive effect that mutates a dependency it reads would otherwise schedule an
// unbounded chain of microtask flushes and hang the page. With proxy-free deep
// reactivity a convergent nested write no longer self-limits via an old===new
// guard (touch() is unconditional), so this per-effect bound is what stops a
// runaway loop — the same safeguard Vue applies (its "Maximum recursive updates"
// limit), counted per effect so a long chain of distinct effects is not aborted.
const MAX_FLUSH_DEPTH = 100;

// bind(c, deps, fn) — register an update function that reads the reactive
// variable indices in `deps`. Runs fn once immediately (correct first paint,
// no flush needed). Returns the bind record (needed for unbind).
export function bind(c, deps, fn) {
  // `runs`/`gen` power the runaway-loop guard (see flush): how many times this
  // record has run within the current flush burst `c.gen`.
  const s = { fn, q: false, alive: true, deps, runs: 0, gen: -1 };
  fn();
  for (const i of deps) (c.deps[i] || (c.deps[i] = [])).push(s);
  if (c.scope) c.scope.subs.push(s);
  return s;
}

// markVar(c, i) — reactive variable i changed: enqueue its dependents
// (deduplicated via the per-record q flag) and schedule a microtask flush.
export function markVar(c, i) {
  const ds = c.deps[i];
  if (ds) {
    for (const s of ds) {
      if (s.alive && !s.q) {
        s.q = true;
        c.queue.push(s);
      }
    }
  }
  if (!c.pending) {
    c.pending = true;
    queueMicrotask(() => flush(c));
  }
}

// flush(c) — run every queued update once. Only affected parts run.
// After the update pass, drain any post-flush callbacks registered via
// `afterFlush` (used by nextTick): they observe the DOM already updated.
export function flush(c) {
  c.pending = false;
  const q = c.queue;
  c.queue = [];
  const ran = q.length > 0;
  for (const s of q) {
    s.q = false;
    if (s.alive) {
      // Runaway-loop guard (per effect). A burst is one chain of self-triggered
      // flushes, tagged by `c.gen`; within it we count how many times THIS record
      // has run. A distinct-effect cascade runs each record ~once, so it never
      // trips — only an effect that keeps re-marking a dependency it reads does.
      // Counting per record (not per pass) is why a long legitimate chain is not
      // falsely aborted.
      if (s.gen !== c.gen) {
        s.gen = c.gen;
        s.runs = 0;
      }
      if (++s.runs > MAX_FLUSH_DEPTH) {
        abortBurst(c, q);
        return;
      }
      s.fn();
    }
  }
  // onUpdate (lifecycle.mjs) — a lightweight per-context post-update hook that
  // fires only when an update pass actually ran, distinct from the one-shot
  // `post` callbacks (afterFlush/nextTick) drained below. Additive: null unless
  // a component registered onUpdate.
  if (ran && c.onUpdate) c.onUpdate();
  const post = c.post;
  if (post) {
    c.post = null;
    for (const cb of post) cb();
  }
  // Burst settled (nothing re-armed a flush): start a fresh generation so the
  // next, unrelated burst counts run-depth from zero.
  if (!c.pending) c.gen++;
}

// abortBurst(c, q) — tear down a runaway update loop cleanly. Clears every queued
// record's `q` flag on BOTH the in-flight queue and any records re-enqueued
// during this pass (so innocent effects sharing a dependency are not left
// permanently un-queueable), drops pending post callbacks (afterFlush/nextTick,
// which could otherwise perpetuate the loop), disarms the pending flush, and
// bumps the generation. A dev warning names the cause.
function abortBurst(c, q) {
  for (const s of q) s.q = false;
  for (const s of c.queue) s.q = false;
  c.queue = [];
  c.post = null;
  c.pending = false;
  c.gen++;
  if (typeof console !== "undefined" && console.warn) {
    console.warn(
      "[lunas] update loop aborted after " +
        MAX_FLUSH_DEPTH +
        " self-triggered passes — a reactive effect keeps mutating a dependency it reads."
    );
  }
}

// afterFlush(c, cb) — run `cb` once, after the next flush completes (i.e. after
// the DOM update pass). If a flush is already pending the callback rides that
// one; otherwise a flush is scheduled so the callback still fires this tick.
// This is the primitive behind nextTick(); it keeps flush the single place that
// knows when updates have landed.
export function afterFlush(c, cb) {
  (c.post || (c.post = [])).push(cb);
  if (!c.pending) {
    c.pending = true;
    queueMicrotask(() => flush(c));
  }
}

// unbind(c, s) — permanently unregister a bind record. Safe to call while a
// flush containing `s` is pending (the dead record is skipped at flush).
export function unbind(c, s) {
  s.alive = false;
  for (const i of s.deps) {
    const ds = c.deps[i];
    if (ds) {
      const k = ds.indexOf(s);
      if (k >= 0) ds.splice(k, 1);
    }
  }
}

// ---------------------------------------------------------------------------
// Scopes — teardown bookkeeping for control-flow blocks.
// beginScope/endScope bracket a block's `make()`; every bind created inside
// (including nested blocks' binds, via nested scopes) is collected so
// dropScope can unregister the whole subtree when the block content is
// removed. Registration cost is one array push per bind.
// ---------------------------------------------------------------------------

export function beginScope(c) {
  // `disposers` hold extra teardown callbacks registered against this scope
  // (e.g. a child component's `unmount`, added by mountChild). They run when
  // the scope is dropped, so a `:for`/`:if` item that mounts a component
  // unmounts it — firing onDestroy and unlinking it from the parent's
  // `_children` — when the item is removed (for-diff-design.md §6).
  const scope = { subs: [], children: [], disposers: null, parent: c.scope };
  if (c.scope) c.scope.children.push(scope);
  c.scope = scope;
  return scope;
}

// addDisposer(c, fn) — register `fn` on the currently-open scope (if any) to be
// run when that scope is dropped. Returns true if it was registered, false when
// no scope is open (the caller then owns teardown itself, e.g. a top-level
// mountChild). Kept null-until-needed so scopes stay cheap.
export function addDisposer(c, fn) {
  const scope = c.scope;
  if (!scope) return false;
  (scope.disposers || (scope.disposers = [])).push(fn);
  return true;
}

export function endScope(c) {
  c.scope = c.scope ? c.scope.parent : null;
}

// runScope(c, scope) — re-run every live bind registered in `scope` and its
// child scopes (nested blocks' content). Used by forBlock's patch path: after
// an item's data cell is updated, re-running the item's scope refreshes every
// item-local bind — including nested ifBlock/forBlock binds, which re-evaluate
// their condition/reconcile with the fresh data (for-diff-design.md §6).
// Scopes dropped mid-walk are safe: dropScope empties their arrays and marks
// their records dead, so they no-op here.
export function runScope(c, scope) {
  // Snapshot first: a sub may create child scopes (a branch shown by this very
  // walk) whose binds already ran at creation, or drop existing ones (their
  // arrays are emptied by dropScope, so recursing into them is a no-op).
  const subs = scope.subs.slice();
  const children = scope.children.slice();
  for (const s of subs) if (s.alive) s.fn();
  for (const ch of children) runScope(c, ch);
}

export function dropScope(c, scope) {
  for (const s of scope.subs) unbind(c, s);
  for (const ch of scope.children) {
    ch.parent = null; // avoid double-splice below
    dropScope(c, ch);
  }
  // Run extra teardown (child-component unmounts) registered on this scope.
  // Guarded to run once: null the list first so a disposer that re-enters
  // (or a double dropScope) can't fire it twice.
  const disposers = scope.disposers;
  if (disposers) {
    scope.disposers = null;
    for (const fn of disposers) fn();
  }
  scope.subs.length = 0;
  scope.children.length = 0;
  if (scope.parent) {
    const sib = scope.parent.children;
    const k = sib.indexOf(scope);
    if (k >= 0) sib.splice(k, 1);
    scope.parent = null;
  }
}
