import { __BLVE_ADD_EV_LISTENER, __BLVE_ESCAPE_HTML, __BLVE_GET_ELM_REFS, __BLVE_INIT_COMPONENT, __BLVE_REPLACE_INNER_HTML, __BLVE_REPLACE_TEXT, __BLVE_REPLACE_ATTR, __BLVE_INSERT_EMPTY, __BLVE_INSERT_CONTENT } from "blve/dist/runtime";

export default function() {
    const { __BLVE_SET_COMPONENT_ELEMENT, __BLVE_UPDATE_COMPONENT, __BLVE_COMPONENT_RETURN, __BLVE_AFTER_MOUNT, __BLVE_REACTIVE } = __BLVE_INIT_COMPONENT();
    let color = __BLVE_REACTIVE('red')
    function yellow(){
      color.v = 'yellow'
    }
    function red(){
      color.v = 'red'
    }
    function blve(){
      color.v = 'blue'
    }
    __BLVE_SET_COMPONENT_ELEMENT(`<span id="hNXqc_j$VSr_JHwwmnANe" style="${`color:${color.v}`}">I am a color</span><button id="yKr_KeWCcdUTyYyxpyHCH">黄色</button><button id="lkIfmqlfoYznPmPwVKiqb">赤色</button><button id="LwywxHlYakgWiuXzdyqGW">青色</button>`, "div");
    __BLVE_AFTER_MOUNT(function () {
        const [__BLVE_SGjusaWNhjaSleIKIAMeO_REF, __BLVE__V_cXkudJBCOFPoLhJeoY_REF, __BLVE_xVBDDHJOomKhfbHPShwkv_REF, __BLVE_EZQovubWKBoPCMUQkkqBq_REF] = __BLVE_GET_ELM_REFS(["hNXqc_j$VSr_JHwwmnANe", "yKr_KeWCcdUTyYyxpyHCH", "lkIfmqlfoYznPmPwVKiqb", "LwywxHlYakgWiuXzdyqGW"], 15);
        __BLVE_ADD_EV_LISTENER(__BLVE__V_cXkudJBCOFPoLhJeoY_REF, "click", yellow);
        __BLVE_ADD_EV_LISTENER(__BLVE_xVBDDHJOomKhfbHPShwkv_REF, "click", red);
        __BLVE_ADD_EV_LISTENER(__BLVE_EZQovubWKBoPCMUQkkqBq_REF, "click", blve);
        this.blkUpdateMap = 0
        __BLVE_UPDATE_COMPONENT(function () {
            this.valUpdateMap & 1 && __BLVE_REPLACE_ATTR("style", `color:${color.v}`, __BLVE_SGjusaWNhjaSleIKIAMeO_REF);
        });
    });
    return __BLVE_COMPONENT_RETURN;
}