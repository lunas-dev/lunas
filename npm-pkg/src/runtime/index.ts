export type BlveComponent = {
  updatedFlag: boolean;
  valUpdateMap: number;
  blkRenderedMap: number;
  blkUpdateMap: number;
  internalElement: BlveInternalElement;
  current_bit: number;
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

export const __BLVE_INIT_COMPONENT = function (this: BlveComponent) {
  this.updatedFlag = false;
  this.valUpdateMap = 0;
  this.blkRenderedMap = 0;
  this.blkUpdateMap = 0;
  this.current_bit = 0;
  this.isMounted = false;

  const genBit = function* (this: BlveComponent) {
    while (true) {
      if (this.current_bit === 0) {
        this.current_bit = 1;
        yield this.current_bit;
      } else {
        this.current_bit <<= 1;
        yield this.current_bit;
      }
    }
  }.bind(this);

  const componentElementSetter = function (
    this: BlveComponent,
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

  const setAfterMount = function (this: BlveComponent, afterMount: () => void) {
    this.__blve_after_mount = afterMount;
  }.bind(this);

  const mount = function (
    this: BlveComponent,
    elm: HTMLElement
  ): BlveComponent {
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

  const insertBefore = function (
    this: BlveComponent,
    elm: HTMLElement,
    anchor: HTMLElement | null
  ): BlveComponent {
    if (this.isMounted) throw new Error("Component is already mounted");
    const componentElm = document.createElement(this.internalElement.topElmTag);
    Object.keys(this.internalElement.topElmAttr).forEach((key) => {
      componentElm.setAttribute(key, this.internalElement.topElmAttr[key]);
    });
    componentElm.innerHTML = this.internalElement.innerHtml;
    elm.insertBefore(componentElm, anchor);
    this.__blve_after_mount();
    this.isMounted = true;
    return this;
  }.bind(this);

  const updateComponent = function (
    this: BlveComponent,
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

  const createReactive = function <T>(this: BlveComponent, v: T) {
    return new valueObj<T>(v, this, genBit().next().value);
  }.bind(this);

  return {
    __BLVE_SET_COMPONENT_ELEMENT: componentElementSetter,
    __BLVE_UPDATE_COMPONENT: updateComponent,
    __BLVE_AFTER_MOUNT: setAfterMount,
    __BLVE_REACTIVE: createReactive,
    __BLVE_COMPONENT_RETURN: {
      mount,
      insertBefore,
    },
  };
}.bind({} as BlveComponent);

class valueObj<T> {
  constructor(
    private _v: T,
    private componentObj: BlveComponent,
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

export function __BLVE_ESCAPE_HTML(text: any): string {
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

export function __BLVE_GET_ELM_REFS(ids: string[], preserveId: number) {
  return ids.map((id, index) => {
    const e = document.getElementById(id)!;
    (2 ** index) & preserveId && e.removeAttribute("id");
    return e;
  });
}

export function __BLVE_ADD_EV_LISTENER(
  elm: HTMLElement,
  evName: string,
  evFunc: EventListener
) {
  elm.addEventListener(evName, evFunc);
}

export function __BLVE_REPLACE_INNER_HTML(content: any, elm: Node) {
  elm.textContent = __BLVE_ESCAPE_HTML(content);
}

export function __BLVE_REPLACE_TEXT(content: any, elm: HTMLElement) {
  elm.innerHTML = __BLVE_ESCAPE_HTML(content);
}

export function __BLVE_REPLACE_ATTR(
  key: string,
  content: any,
  elm: HTMLElement
) {
  if (content === undefined && elm.hasAttribute(key)) {
    elm.removeAttribute(key);
    return;
  }
  (elm as any)[key] = String(content);
}

export function __BLVE_INSERT_EMPTY(
  parent: HTMLElement,
  anchor: HTMLElement | null
) {
  const empty = document.createTextNode(" ");
  parent.insertBefore(empty, anchor);
  return empty;
}

export function __BLVE_INSERT_CONTENT(
  content: string,
  parent: HTMLElement,
  anchor: HTMLElement | null
) {
  const contentNode = document.createTextNode(content);
  parent.insertBefore(contentNode, anchor);
  return contentNode;
}
