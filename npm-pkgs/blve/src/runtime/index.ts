export type ComponentDeclaration = (args?: {
  [key: string]: any;
}) => BlveModuleExports;

export type BlveModuleExports = {
  mount: (elm: HTMLElement) => BlveComponentState;
  insert: (elm: HTMLElement, anchor: HTMLElement | null) => BlveComponentState;
};

export type BlveComponentState = {
  updatedFlag: boolean;
  valUpdateMap: number;
  blkRenderedMap: number;
  blkUpdateMap: number;
  internalElement: BlveInternalElement;
  currentVarBit: number;
  currentIfBlkBit: number;
  ifBlkRenderers: { [key: string]: () => void };
  isMounted: boolean;
  // componentElmentSetter: (innerHtml: string, topElmTag: string,topElmAttr: {[key: string]: string}) => void
  __blve_update: () => void;
  __blve_after_mount: () => void;
  // __blve_init: () => void;
  // __blve_destroy: () => void;
  // __blve_update_component: () => void;
  // __blve_update_component_end: () => void;
  // __blve_update_component_start: () => void;
  // __blve_update_end: () => void;
  // __blve_update_start: () => void;
  // __blve_init_component: () => void;
};

type BlveInternalElement = {
  innerHtml: string;
  topElmTag: string;
  topElmAttr: { [key: string]: string };
};

export const $$blveInitComponent = function (this: BlveComponentState) {
  this.updatedFlag = false;
  this.valUpdateMap = 0;
  this.blkRenderedMap = 0;
  this.blkUpdateMap = 0;
  this.currentVarBit = 0;
  this.currentIfBlkBit = 0;
  this.isMounted = false;
  this.ifBlkRenderers = {};

  const genBitOfVariables = function* (this: BlveComponentState) {
    while (true) {
      if (this.currentVarBit === 0) {
        this.currentVarBit = 1;
        yield this.currentVarBit;
      } else {
        this.currentVarBit <<= 1;
        yield this.currentVarBit;
      }
    }
  }.bind(this);

  const genBitOfIfBlks = function* (this: BlveComponentState) {
    while (true) {
      if (this.currentIfBlkBit === 0) {
        this.currentIfBlkBit = 1;
        yield this.currentIfBlkBit;
      } else {
        this.currentIfBlkBit <<= 1;
        yield this.currentIfBlkBit;
      }
    }
  }.bind(this);

  const componentElementSetter = function (
    this: BlveComponentState,
    innerHtml: string,
    topElmTag: string,
    topElmAttr: { [key: string]: string } = {}
  ) {
    this.internalElement = {
      innerHtml,
      topElmTag,
      topElmAttr,
    };
  }.bind(this);

  const setAfterMount = function (
    this: BlveComponentState,
    afterMount: () => void
  ) {
    this.__blve_after_mount = afterMount;
  }.bind(this);

  const mount = function (
    this: BlveComponentState,
    elm: HTMLElement
  ): BlveComponentState {
    if (this.isMounted) throw new Error("Component is already mounted");
    elm.innerHTML = `<${this.internalElement.topElmTag} ${Object.keys(
      this.internalElement.topElmAttr
    )
      .map((key) => `${key}="${this.internalElement.topElmAttr[key]}"`)
      .join(" ")}>${this.internalElement.innerHtml}</${
      this.internalElement.topElmTag
    }>`;
    this.__blve_after_mount();
    this.isMounted = true;
    return this;
  }.bind(this);

  const insert = function (
    this: BlveComponentState,
    elm: HTMLElement,
    anchor: HTMLElement | null
  ): BlveComponentState {
    if (this.isMounted) throw new Error("Component is already mounted");
    const componentElm = createDomElementFromBlveElement(this.internalElement);
    elm.insertBefore(componentElm, anchor);
    this.__blve_after_mount();
    this.isMounted = true;
    return this;
  }.bind(this);

  const updateComponent = function (
    this: BlveComponentState,
    updateFunc: () => void
  ) {
    this.__blve_update = (() => {
      if (!this.updatedFlag) return;
      updateFunc.call(this);
      this.updatedFlag = false;
      this.valUpdateMap = 0;
      this.blkUpdateMap = 0;
    }).bind(this);
  }.bind(this);

  const createReactive = function <T>(this: BlveComponentState, v: T) {
    return new valueObj<T>(v, this, genBitOfVariables().next().value);
  }.bind(this);

  const createIfBlock = function (
    this: BlveComponentState,
    name: string,
    blveElement: () => BlveInternalElement,
    getParentAndRefElement: () => [HTMLElement, HTMLElement | null],
    postRender: () => void
  ) {
    const ifBlkBit = genBitOfIfBlks().next().value;
    this.ifBlkRenderers[name] = (() => {
      const componentElm = createDomElementFromBlveElement(blveElement());
      const [parentElement, refElement] = getParentAndRefElement();
      parentElement.insertBefore(componentElm, refElement);
      postRender();
      (this.blkRenderedMap |= ifBlkBit), (this.blkUpdateMap |= ifBlkBit);
    }).bind(this);
  }.bind(this);

  const renderIfBlock = function (this: BlveComponentState, name: string) {
    if (!this.ifBlkRenderers[name]) return;
    this.ifBlkRenderers[name]();
  }.bind(this);

  return {
    $$blveSetComponentElement: componentElementSetter,
    $$blveUpdateComponent: updateComponent,
    $$blveAfterMount: setAfterMount,
    $$blveReactive: createReactive,
    $$blveCreateIfBlock: createIfBlock,
    $$blveRenderIfBlock: renderIfBlock,
    $$blveComponentReturn: {
      mount,
      insert,
    } as BlveModuleExports,
  };
};

class valueObj<T> {
  constructor(
    private _v: T,
    private componentObj: BlveComponentState,
    private symbolIndex: number
  ) {}

  set v(v: T) {
    if (this._v === v) return;
    this._v = v;
    this.componentObj.valUpdateMap |= this.symbolIndex;
    if (!this.componentObj.updatedFlag) {
      Promise.resolve().then(this.componentObj.__blve_update.bind(this));
      this.componentObj.updatedFlag = true;
    }
  }

  get v() {
    return this._v;
  }
}

export function $$blveEscapeHtml(text: any): string {
  const map: { [key: string]: string } = {
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
    "'": "&#039;",
  };

  return String(text).replace(/[&<>"']/g, function (m: string): string {
    return map[m];
  });
}

export function $$blveGetElmRefs(ids: string[], preserveId: number) {
  return ids.map((id, index) => {
    const e = document.getElementById(id)!;
    (2 ** index) & preserveId && e.removeAttribute("id");
    return e;
  });
}

export function $$blveAddEvListener(
  elm: HTMLElement,
  evName: string,
  evFunc: EventListener
) {
  elm.addEventListener(evName, evFunc);
}

export function $$blveReplaceText(content: any, elm: Node) {
  elm.textContent = $$blveEscapeHtml(content);
}

export function $$blveReplaceInnerHtml(content: any, elm: HTMLElement) {
  elm.innerHTML = $$blveEscapeHtml(content);
}

export function $$blveReplaceAttr(key: string, content: any, elm: HTMLElement) {
  if (content === undefined && elm.hasAttribute(key)) {
    elm.removeAttribute(key);
    return;
  }
  (elm as any)[key] = String(content);
}

export function $$blveInsertEmpty(
  parent: HTMLElement,
  anchor: HTMLElement | null
) {
  const empty = document.createTextNode(" ");
  parent.insertBefore(empty, anchor);
  return empty;
}

export function $$blveInsertContent(
  content: string,
  parent: HTMLElement,
  anchor: HTMLElement | null
) {
  const contentNode = document.createTextNode(content);
  parent.insertBefore(contentNode, anchor);
  return contentNode;
}

export function $$createBlveElement(
  innerHtml: string,
  topElmTag: string,
  topElmAttr: { [key: string]: string }
): BlveInternalElement {
  return {
    innerHtml,
    topElmTag,
    topElmAttr,
  };
}

export const createDomElementFromBlveElement = function (
  blveElement: BlveInternalElement
): HTMLElement {
  const componentElm = document.createElement(blveElement.topElmTag);
  Object.keys(blveElement.topElmAttr).forEach((key) => {
    componentElm.setAttribute(key, blveElement.topElmAttr[key]);
  });
  componentElm.innerHTML = blveElement.innerHtml;
  return componentElm;
};
