import { __BLVE_ADD_EV_LISTENER, __BLVE_ESCAPE_HTML, __BLVE_GET_ELM_REFS, __BLVE_INIT_COMPONENT, __BLVE_REPLACE_INNER_HTML, __BLVE_REPLACE_TEXT, __BLVE_REPLACE_ATTR, __BLVE_INSERT_EMPTY, __BLVE_INSERT_CONTENT } from "blve/dist/runtime";

export default function() {
    const { __BLVE_SET_COMPONENT_ELEMENT, __BLVE_UPDATE_COMPONENT, __BLVE_COMPONENT_RETURN, __BLVE_AFTER_MOUNT, __BLVE_REACTIVE } = __BLVE_INIT_COMPONENT();
    let boolVal1 = __BLVE_REACTIVE(false)
    function toggle() {
      boolVal1.v = !boolVal1.v
    }
    __BLVE_SET_COMPONENT_ELEMENT(`<div id="WUpPtHUBjCrfqyDwDhGzL">AAA</div><button id="EDZocVZQN$zjsPZ$qPZbm">toggle</button><div id="AWeMABHBqZCuzpFfcRcCa">${__BLVE_ESCAPE_HTML(boolVal1.v)}</div>`, "div", {id: "_forhNDTXhMiXqfFWTKxt",});
    __BLVE_AFTER_MOUNT(function () {
        const [__BLVE_efWvRfeSxzfJXhzTdShEb_REF, __BLVE_dUhf$oMo$uxUJz$CplFWy_REF, __BLVE_vfNHZkIDnmFpYLQcnsRfe_REF, __BLVE_HsyNdNVIqwqfttDbIg$$w_REF] = __BLVE_GET_ELM_REFS(["EDZocVZQN$zjsPZ$qPZbm", "AWeMABHBqZCuzpFfcRcCa", "_forhNDTXhMiXqfFWTKxt", "WUpPtHUBjCrfqyDwDhGzL"], 15);
        let __BLVE_GamcKWqwrHXbxxZAvKJHH_REF;
        __BLVE_ADD_EV_LISTENER(__BLVE_efWvRfeSxzfJXhzTdShEb_REF, "click", toggle);
        const __BLVE_RENDER_GamcKWqwrHXbxxZAvKJHH_ELM = () => {
            __BLVE_GamcKWqwrHXbxxZAvKJHH_REF = document.createElement("div");
            __BLVE_GamcKWqwrHXbxxZAvKJHH_REF.innerHTML = `AAA`;
            __BLVE_vfNHZkIDnmFpYLQcnsRfe_REF.insertBefore(__BLVE_GamcKWqwrHXbxxZAvKJHH_REF, __BLVE_HsyNdNVIqwqfttDbIg$$w_REF);
            this.blkRenderedMap |= 1, this.blkUpdateMap |= 1;
        }
        boolVal1.v && __BLVE_RENDER_GamcKWqwrHXbxxZAvKJHH_ELM()
        this.blkUpdateMap = 0
        __BLVE_UPDATE_COMPONENT(function () {
            this.valUpdateMap & 1 && ( boolVal1.v ? __BLVE_RENDER_GamcKWqwrHXbxxZAvKJHH_ELM() : (__BLVE_GamcKWqwrHXbxxZAvKJHH_REF.remove(), __BLVE_GamcKWqwrHXbxxZAvKJHH_REF = null, this.blkRenderedMap ^= 1) );
            this.valUpdateMap & 1 && __BLVE_REPLACE_TEXT(`${__BLVE_ESCAPE_HTML(boolVal1.v)}`, __BLVE_dUhf$oMo$uxUJz$CplFWy_REF);
        });
    });
    return __BLVE_COMPONENT_RETURN;
}