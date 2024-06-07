import { __BLVE_ADD_EV_LISTENER, __BLVE_ESCAPE_HTML, __BLVE_GET_ELM_REFS, __BLVE_INIT_COMPONENT, __BLVE_REPLACE_INNER_HTML, __BLVE_REPLACE_TEXT, __BLVE_REPLACE_ATTR, __BLVE_INSERT_EMPTY, __BLVE_INSERT_CONTENT } from "blve/dist/runtime";

export default function() {
    const { __BLVE_SET_COMPONENT_ELEMENT, __BLVE_UPDATE_COMPONENT, __BLVE_COMPONENT_RETURN, __BLVE_AFTER_MOUNT, __BLVE_REACTIVE } = __BLVE_INIT_COMPONENT();
    const inputValue = __BLVE_REACTIVE("")
    function setFoo() {
      inputValue.v = "foo"
    }
    __BLVE_SET_COMPONENT_ELEMENT(`<input id="hyDvUFpBXTawervJGaTte" /><div id="Lnd_NTiIdUuISofwdrcfg">inputValue: ${__BLVE_ESCAPE_HTML( inputValue.v )}</div><button id="RmLUCoG$cpyVbsOddOylu">set foo</button>`, "div");
    __BLVE_AFTER_MOUNT(function () {
        const [__BLVE_liYxUV$NdYOOLuzmOwmok_REF, __BLVE_meqKOzkzNafrdxZnkfLjG_REF, __BLVE_S_mSGldziZRSWWICfLSgL_REF] = __BLVE_GET_ELM_REFS(["hyDvUFpBXTawervJGaTte", "Lnd_NTiIdUuISofwdrcfg", "RmLUCoG$cpyVbsOddOylu"], 7);
        __BLVE_ADD_EV_LISTENER(__BLVE_liYxUV$NdYOOLuzmOwmok_REF, "input", (e)=>inputValue.v = event.target.value);
        __BLVE_ADD_EV_LISTENER(__BLVE_S_mSGldziZRSWWICfLSgL_REF, "click", setFoo);
        this.blkUpdateMap = 0
        __BLVE_UPDATE_COMPONENT(function () {
            this.valUpdateMap & 1 && __BLVE_REPLACE_ATTR("value", inputValue.v, __BLVE_liYxUV$NdYOOLuzmOwmok_REF);
            this.valUpdateMap & 1 && __BLVE_REPLACE_TEXT(`inputValue: ${__BLVE_ESCAPE_HTML( inputValue.v )}`, __BLVE_meqKOzkzNafrdxZnkfLjG_REF);
        });
    });
    return __BLVE_COMPONENT_RETURN;
}