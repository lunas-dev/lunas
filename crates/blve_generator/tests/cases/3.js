import { __BLVE_ADD_EV_LISTENER, __BLVE_ESCAPE_HTML, __BLVE_GET_ELM_REFS, __BLVE_INIT_COMPONENT, __BLVE_REPLACE_INNER_HTML, __BLVE_REPLACE_TEXT, __BLVE_REPLACE_ATTR, __BLVE_INSERT_EMPTY, __BLVE_INSERT_CONTENT } from "blve/dist/runtime";

export default function() {
    const { __BLVE_SET_COMPONENT_ELEMENT, __BLVE_UPDATE_COMPONENT, __BLVE_COMPONENT_RETURN, __BLVE_AFTER_MOUNT, __BLVE_REACTIVE } = __BLVE_INIT_COMPONENT();
    const balue = __BLVE_REACTIVE("")
    function onInput(e) {
      balue.v = e.target.value
    }
    function xxx() {
      balue.v += "xxx"
    }
    __BLVE_SET_COMPONENT_ELEMENT(`<input id="mSZVkKzedMphuVkGNIHeF" value="${balue.v}" /><div id="tzPQlyiTuzVvo_BLFwCqV">${__BLVE_ESCAPE_HTML(balue.v)}</div><button id="syHUaALEsaXMHcAsLMvyE">xxx</button>`, "div");
    __BLVE_AFTER_MOUNT(function () {
        const [__BLVE_SRQTSFHgkUDmxbSQzOpaC_REF, __BLVE_teUOt$QowSEAoQtfKakRM_REF, __BLVE_ByksPBexDrevhTYa$JttX_REF] = __BLVE_GET_ELM_REFS(["mSZVkKzedMphuVkGNIHeF", "tzPQlyiTuzVvo_BLFwCqV", "syHUaALEsaXMHcAsLMvyE"], 7);
        __BLVE_ADD_EV_LISTENER(__BLVE_SRQTSFHgkUDmxbSQzOpaC_REF, "input", onInput);
        __BLVE_ADD_EV_LISTENER(__BLVE_ByksPBexDrevhTYa$JttX_REF, "click", xxx);
        this.blkUpdateMap = 0
        __BLVE_UPDATE_COMPONENT(function () {
            this.valUpdateMap & 1 && __BLVE_REPLACE_ATTR("value", balue.v, __BLVE_SRQTSFHgkUDmxbSQzOpaC_REF);
            this.valUpdateMap & 1 && __BLVE_REPLACE_TEXT(`${__BLVE_ESCAPE_HTML(balue.v)}`, __BLVE_teUOt$QowSEAoQtfKakRM_REF);
        });
    });
    return __BLVE_COMPONENT_RETURN;
}