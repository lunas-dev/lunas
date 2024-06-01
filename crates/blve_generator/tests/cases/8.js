import { __BLVE_ADD_EV_LISTENER, __BLVE_ESCAPE_HTML, __BLVE_GET_ELM_REFS, __BLVE_INIT_COMPONENT, __BLVE_REPLACE_INNER_HTML, __BLVE_REPLACE_TEXT, __BLVE_REPLACE_ATTR, __BLVE_INSERT_EMPTY, __BLVE_INSERT_CONTENT } from "blve/dist/runtime";

export default function() {
    const { __BLVE_SET_COMPONENT_ELEMENT, __BLVE_UPDATE_COMPONENT, __BLVE_COMPONENT_RETURN, __BLVE_AFTER_MOUNT, __BLVE_REACTIVE } = __BLVE_INIT_COMPONENT();
    let boolVal1 = __BLVE_REACTIVE(Math.random() > 0.5)
    let boolVal2 = __BLVE_REACTIVE(Math.random() > 0.5)
    let boolVal3 = __BLVE_REACTIVE(Math.random() > 0.5)
    let boolVal4 = __BLVE_REACTIVE(Math.random() > 0.5)
    let boolVal5 = __BLVE_REACTIVE(Math.random() > 0.5)
    let boolVal6 = __BLVE_REACTIVE(Math.random() > 0.5)
    function toggle(){
      boolVal1.v = !boolVal1.v
      boolVal2.v = !boolVal2.v
      boolVal3.v = !boolVal3.v
      boolVal4.v = !boolVal4.v
      boolVal5.v = !boolVal5.v
      boolVal6.v = !boolVal6.v
    }
    __BLVE_SET_COMPONENT_ELEMENT(`<div id="bfBi_AdyQbxpmGujhPyNi"><div id="nnXlXLNTzhEjOjH_mLaPH">Static</div></div><button id="VOq_FowHMBMXHXB_YL_wi">toggle</button>`, "div");
    __BLVE_AFTER_MOUNT(function () {
        const [__BLVE_FPDiYGKu$gpmeKZzjw_wa_REF, __BLVE_rJnoFMRBAbExY$PpTUAcw_REF, __BLVE_TnIKfpalfu_TWCwuPVCQT_REF] = __BLVE_GET_ELM_REFS(["bfBi_AdyQbxpmGujhPyNi", "nnXlXLNTzhEjOjH_mLaPH", "VOq_FowHMBMXHXB_YL_wi"], 7);
        let __BLVE_VzvsqhLIqDZW_OaoRKVcZ_REF, __BLVE_XebjWskhZx__DmYaHSonK_REF, __BLVE_hoYjNySiVoGuRkSwvEhky_REF, __BLVE_rdHlle_QDOqISDNfWxtSL_REF, __BLVE_XFXHxGWxitoxuitksqcmv_REF, __BLVE_JhejclsYeRUtQjLurAJhY_REF;
        const __BLVE_XebjWskhZx__DmYaHSonK_Anchor = __BLVE_INSERT_EMPTY(__BLVE_FPDiYGKu$gpmeKZzjw_wa_REF,__BLVE_rJnoFMRBAbExY$PpTUAcw_REF);
        const __BLVE_XFXHxGWxitoxuitksqcmv_Anchor = __BLVE_INSERT_EMPTY(__BLVE_FPDiYGKu$gpmeKZzjw_wa_REF,__BLVE_rJnoFMRBAbExY$PpTUAcw_REF);
        const __BLVE_hoYjNySiVoGuRkSwvEhky_Anchor = __BLVE_INSERT_EMPTY(__BLVE_FPDiYGKu$gpmeKZzjw_wa_REF,null);
        const __BLVE_JhejclsYeRUtQjLurAJhY_Anchor = __BLVE_INSERT_EMPTY(__BLVE_FPDiYGKu$gpmeKZzjw_wa_REF,null);
        __BLVE_ADD_EV_LISTENER(__BLVE_TnIKfpalfu_TWCwuPVCQT_REF, "click", toggle);
        const __BLVE_RENDER_XebjWskhZx__DmYaHSonK_ELM = () => {
            __BLVE_XebjWskhZx__DmYaHSonK_REF = document.createElement("div");
            __BLVE_XebjWskhZx__DmYaHSonK_REF.innerHTML = `AAA`;
            __BLVE_FPDiYGKu$gpmeKZzjw_wa_REF.insertBefore(__BLVE_XebjWskhZx__DmYaHSonK_REF, __BLVE_XebjWskhZx__DmYaHSonK_Anchor);
            this.blkRenderedMap |= 1, this.blkUpdateMap |= 1;
        }
        boolVal1.v && __BLVE_RENDER_XebjWskhZx__DmYaHSonK_ELM()
        const __BLVE_RENDER_XFXHxGWxitoxuitksqcmv_ELM = () => {
            __BLVE_XFXHxGWxitoxuitksqcmv_REF = document.createElement("div");
            __BLVE_XFXHxGWxitoxuitksqcmv_REF.innerHTML = `BBB`;
            __BLVE_FPDiYGKu$gpmeKZzjw_wa_REF.insertBefore(__BLVE_XFXHxGWxitoxuitksqcmv_REF, __BLVE_XFXHxGWxitoxuitksqcmv_Anchor);
            this.blkRenderedMap |= 2, this.blkUpdateMap |= 2;
        }
        boolVal2.v && __BLVE_RENDER_XFXHxGWxitoxuitksqcmv_ELM()
        const __BLVE_RENDER_rdHlle_QDOqISDNfWxtSL_ELM = () => {
            __BLVE_rdHlle_QDOqISDNfWxtSL_REF = document.createElement("div");
            __BLVE_rdHlle_QDOqISDNfWxtSL_REF.innerHTML = `CCC`;
            __BLVE_FPDiYGKu$gpmeKZzjw_wa_REF.insertBefore(__BLVE_rdHlle_QDOqISDNfWxtSL_REF, __BLVE_rJnoFMRBAbExY$PpTUAcw_REF);
            this.blkRenderedMap |= 4, this.blkUpdateMap |= 4;
        }
        boolVal3.v && __BLVE_RENDER_rdHlle_QDOqISDNfWxtSL_ELM()
        const __BLVE_RENDER_hoYjNySiVoGuRkSwvEhky_ELM = () => {
            __BLVE_hoYjNySiVoGuRkSwvEhky_REF = document.createElement("div");
            __BLVE_hoYjNySiVoGuRkSwvEhky_REF.innerHTML = `DDD`;
            __BLVE_FPDiYGKu$gpmeKZzjw_wa_REF.insertBefore(__BLVE_hoYjNySiVoGuRkSwvEhky_REF, __BLVE_hoYjNySiVoGuRkSwvEhky_Anchor);
            this.blkRenderedMap |= 8, this.blkUpdateMap |= 8;
        }
        boolVal4.v && __BLVE_RENDER_hoYjNySiVoGuRkSwvEhky_ELM()
        const __BLVE_RENDER_JhejclsYeRUtQjLurAJhY_ELM = () => {
            __BLVE_JhejclsYeRUtQjLurAJhY_REF = document.createElement("div");
            __BLVE_JhejclsYeRUtQjLurAJhY_REF.innerHTML = `EEE`;
            __BLVE_FPDiYGKu$gpmeKZzjw_wa_REF.insertBefore(__BLVE_JhejclsYeRUtQjLurAJhY_REF, __BLVE_JhejclsYeRUtQjLurAJhY_Anchor);
            this.blkRenderedMap |= 16, this.blkUpdateMap |= 16;
        }
        boolVal5.v && __BLVE_RENDER_JhejclsYeRUtQjLurAJhY_ELM()
        const __BLVE_RENDER_VzvsqhLIqDZW_OaoRKVcZ_ELM = () => {
            __BLVE_VzvsqhLIqDZW_OaoRKVcZ_REF = document.createElement("div");
            __BLVE_VzvsqhLIqDZW_OaoRKVcZ_REF.innerHTML = `FFF`;
            __BLVE_FPDiYGKu$gpmeKZzjw_wa_REF.insertBefore(__BLVE_VzvsqhLIqDZW_OaoRKVcZ_REF, null);
            this.blkRenderedMap |= 32, this.blkUpdateMap |= 32;
        }
        boolVal6.v && __BLVE_RENDER_VzvsqhLIqDZW_OaoRKVcZ_ELM()
        this.blkUpdateMap = 0
        __BLVE_UPDATE_COMPONENT(function () {
            this.valUpdateMap & 1 && ( boolVal1.v ? __BLVE_RENDER_XebjWskhZx__DmYaHSonK_ELM() : (__BLVE_XebjWskhZx__DmYaHSonK_REF.remove(), __BLVE_XebjWskhZx__DmYaHSonK_REF = null, this.blkRenderedMap ^= 1) );
            this.valUpdateMap & 2 && ( boolVal2.v ? __BLVE_RENDER_XFXHxGWxitoxuitksqcmv_ELM() : (__BLVE_XFXHxGWxitoxuitksqcmv_REF.remove(), __BLVE_XFXHxGWxitoxuitksqcmv_REF = null, this.blkRenderedMap ^= 2) );
            this.valUpdateMap & 4 && ( boolVal3.v ? __BLVE_RENDER_rdHlle_QDOqISDNfWxtSL_ELM() : (__BLVE_rdHlle_QDOqISDNfWxtSL_REF.remove(), __BLVE_rdHlle_QDOqISDNfWxtSL_REF = null, this.blkRenderedMap ^= 3) );
            this.valUpdateMap & 8 && ( boolVal4.v ? __BLVE_RENDER_hoYjNySiVoGuRkSwvEhky_ELM() : (__BLVE_hoYjNySiVoGuRkSwvEhky_REF.remove(), __BLVE_hoYjNySiVoGuRkSwvEhky_REF = null, this.blkRenderedMap ^= 4) );
            this.valUpdateMap & 16 && ( boolVal5.v ? __BLVE_RENDER_JhejclsYeRUtQjLurAJhY_ELM() : (__BLVE_JhejclsYeRUtQjLurAJhY_REF.remove(), __BLVE_JhejclsYeRUtQjLurAJhY_REF = null, this.blkRenderedMap ^= 5) );
            this.valUpdateMap & 32 && ( boolVal6.v ? __BLVE_RENDER_VzvsqhLIqDZW_OaoRKVcZ_ELM() : (__BLVE_VzvsqhLIqDZW_OaoRKVcZ_REF.remove(), __BLVE_VzvsqhLIqDZW_OaoRKVcZ_REF = null, this.blkRenderedMap ^= 6) );
        });
    });
    return __BLVE_COMPONENT_RETURN;
}