import { __BLVE_ADD_EV_LISTENER, __BLVE_ESCAPE_HTML, __BLVE_GET_ELM_REFS, __BLVE_INIT_COMPONENT, __BLVE_REPLACE_INNER_HTML, __BLVE_REPLACE_TEXT, __BLVE_REPLACE_ATTR, __BLVE_INSERT_EMPTY, __BLVE_INSERT_CONTENT } from "blve/dist/runtime";

export default function() {
    const { __BLVE_SET_COMPONENT_ELEMENT, __BLVE_UPDATE_COMPONENT, __BLVE_COMPONENT_RETURN, __BLVE_AFTER_MOUNT, __BLVE_REACTIVE } = __BLVE_INIT_COMPONENT();
    let count = __BLVE_REACTIVE(0)
    function increment(){
      count.v++
      console.log(count.v)
    }
    function clear(){
      if(interval.v){
        clearInterval(interval.v)
        interval.v = null
      }else{
        interval.v = setInterval(increment, 2000)
      }
    }
    let interval = __BLVE_REACTIVE(setInterval(increment, 2000))
    __BLVE_SET_COMPONENT_ELEMENT(`<h1 id="abc">Hello Blve!</h1><div id="EU$ACyJdPgqXwgTtZFQdm">${__BLVE_ESCAPE_HTML(count.v)}</div><button id="A_XVMHZjeJfEwLDOaYHGa">+1</button><button id="bmDvolUlQvqeLETyLNwHt">${__BLVE_ESCAPE_HTML(interval.v==null?"start":"clear")}</button>`, "div");
    __BLVE_AFTER_MOUNT(function () {
        const [__BLVE_BvUTwTUUCOzgEQOQGINYX_REF, __BLVE_YpNYCapwGYaTTITocTopl_REF, __BLVE_zdNCnnUpwhxBwRsENjsb__REF] = __BLVE_GET_ELM_REFS(["EU$ACyJdPgqXwgTtZFQdm", "A_XVMHZjeJfEwLDOaYHGa", "bmDvolUlQvqeLETyLNwHt"], 7);
        __BLVE_ADD_EV_LISTENER(__BLVE_YpNYCapwGYaTTITocTopl_REF, "click", increment);
        __BLVE_ADD_EV_LISTENER(__BLVE_zdNCnnUpwhxBwRsENjsb__REF, "click", clear);
        this.blkUpdateMap = 0
        __BLVE_UPDATE_COMPONENT(function () {
            this.valUpdateMap & 1 && __BLVE_REPLACE_TEXT(`${__BLVE_ESCAPE_HTML(count.v)}`, __BLVE_BvUTwTUUCOzgEQOQGINYX_REF);
            this.valUpdateMap & 2 && __BLVE_REPLACE_TEXT(`${__BLVE_ESCAPE_HTML(interval.v==null?"start":"clear")}`, __BLVE_zdNCnnUpwhxBwRsENjsb__REF);
        });
    });
    return __BLVE_COMPONENT_RETURN;
}