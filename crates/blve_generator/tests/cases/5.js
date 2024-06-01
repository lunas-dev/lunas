import { __BLVE_ADD_EV_LISTENER, __BLVE_ESCAPE_HTML, __BLVE_GET_ELM_REFS, __BLVE_INIT_COMPONENT, __BLVE_REPLACE_INNER_HTML, __BLVE_REPLACE_TEXT, __BLVE_REPLACE_ATTR, __BLVE_INSERT_EMPTY, __BLVE_INSERT_CONTENT } from "blve/dist/runtime";

export default function() {
    const { __BLVE_SET_COMPONENT_ELEMENT, __BLVE_UPDATE_COMPONENT, __BLVE_COMPONENT_RETURN, __BLVE_AFTER_MOUNT, __BLVE_REACTIVE } = __BLVE_INIT_COMPONENT();
    let inputValue = __BLVE_REACTIVE("")
    function addFoo() {
      inputValue.v += "foo"
    }
    __BLVE_SET_COMPONENT_ELEMENT(`<input id="eSpbDaslllcKBSCPpkgxM" /><div id="rGalXBsYRiNTicjdyIpJU">inputValue: ${__BLVE_ESCAPE_HTML( inputValue.v )}</div><button id="PboOoVHwCYqav$lckcooZ">setFoo</button><button id="ZcXiiemMeYLClwsdk_zbO">addFoo1</button><button id="wOdwZyaEOeaKSydaikEdt">addFoo2</button>`, "div");
    __BLVE_AFTER_MOUNT(function () {
        const [__BLVE_YEHvYSiEoagnBuLDAMylN_REF, __BLVE_V$IeIt$BWxpi$BpXSRWkP_REF, __BLVE_mllfWWJkwXQxXXPylxo_W_REF, __BLVE_ZTGFfmPrISmqmQQjqbMlo_REF, __BLVE_BjEIWRyzCjUbksRouxeuP_REF] = __BLVE_GET_ELM_REFS(["eSpbDaslllcKBSCPpkgxM", "rGalXBsYRiNTicjdyIpJU", "PboOoVHwCYqav$lckcooZ", "ZcXiiemMeYLClwsdk_zbO", "wOdwZyaEOeaKSydaikEdt"], 31);
        __BLVE_ADD_EV_LISTENER(__BLVE_YEHvYSiEoagnBuLDAMylN_REF, "input", (e)=>inputValue.v = event.target.value);
        __BLVE_ADD_EV_LISTENER(__BLVE_mllfWWJkwXQxXXPylxo_W_REF, "click", ()=>inputValue.v = 'foo');
        __BLVE_ADD_EV_LISTENER(__BLVE_ZTGFfmPrISmqmQQjqbMlo_REF, "click", addFoo);
        __BLVE_ADD_EV_LISTENER(__BLVE_BjEIWRyzCjUbksRouxeuP_REF, "click", ()=>addFoo());
        this.blkUpdateMap = 0
        __BLVE_UPDATE_COMPONENT(function () {
            this.valUpdateMap & 1 && __BLVE_REPLACE_ATTR("value", inputValue.v, __BLVE_YEHvYSiEoagnBuLDAMylN_REF);
            this.valUpdateMap & 1 && __BLVE_REPLACE_TEXT(`inputValue: ${__BLVE_ESCAPE_HTML( inputValue.v )}`, __BLVE_V$IeIt$BWxpi$BpXSRWkP_REF);
        });
    });
    return __BLVE_COMPONENT_RETURN;
}