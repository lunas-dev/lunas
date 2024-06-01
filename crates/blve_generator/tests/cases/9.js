import { __BLVE_ADD_EV_LISTENER, __BLVE_ESCAPE_HTML, __BLVE_GET_ELM_REFS, __BLVE_INIT_COMPONENT, __BLVE_REPLACE_INNER_HTML, __BLVE_REPLACE_TEXT, __BLVE_REPLACE_ATTR, __BLVE_INSERT_EMPTY, __BLVE_INSERT_CONTENT } from "blve/dist/runtime";

export default function() {
    const { __BLVE_SET_COMPONENT_ELEMENT, __BLVE_UPDATE_COMPONENT, __BLVE_COMPONENT_RETURN, __BLVE_AFTER_MOUNT, __BLVE_REACTIVE } = __BLVE_INIT_COMPONENT();
    let showBlock = __BLVE_REACTIVE(true)
    function toggle() {
      showBlock.v = !showBlock.v
    }
    __BLVE_SET_COMPONENT_ELEMENT(``, "div", {id: "tkImxPylvz_lkHpfAnQdb",});
    __BLVE_AFTER_MOUNT(function () {
        const [__BLVE_lMRhGzfsO_iFWSvCRBlUo_REF] = __BLVE_GET_ELM_REFS(["tkImxPylvz_lkHpfAnQdb"], 1);
        let __BLVE_OlNdvhscaQqrghWIlCAWe_REF;
        const __BLVE_RENDER_OlNdvhscaQqrghWIlCAWe_ELM = () => {
            __BLVE_OlNdvhscaQqrghWIlCAWe_REF = document.createElement("div");
            __BLVE_OlNdvhscaQqrghWIlCAWe_REF.innerHTML = `
                THIS IS IF BLOCK1
              `;
            __BLVE_lMRhGzfsO_iFWSvCRBlUo_REF.insertBefore(__BLVE_OlNdvhscaQqrghWIlCAWe_REF, null);
            this.blkRenderedMap |= 1, this.blkUpdateMap |= 1;
        }
        showBlock.v && __BLVE_RENDER_OlNdvhscaQqrghWIlCAWe_ELM()
        this.blkUpdateMap = 0
        __BLVE_UPDATE_COMPONENT(function () {
            this.valUpdateMap & 1 && ( showBlock.v ? __BLVE_RENDER_OlNdvhscaQqrghWIlCAWe_ELM() : (__BLVE_OlNdvhscaQqrghWIlCAWe_REF.remove(), __BLVE_OlNdvhscaQqrghWIlCAWe_REF = null, this.blkRenderedMap ^= 1) );
        });
    });
    return __BLVE_COMPONENT_RETURN;
}