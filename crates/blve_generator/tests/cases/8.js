
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let boolVal1 = reactiveValue(Math.random() > 0.5, 1, refs)
    let boolVal2 = reactiveValue(Math.random() > 0.5, 2, refs)
    let boolVal3 = reactiveValue(Math.random() > 0.5, 4, refs)
    let boolVal4 = reactiveValue(Math.random() > 0.5, 8, refs)
    let boolVal5 = reactiveValue(Math.random() > 0.5, 16, refs)
    let boolVal6 = reactiveValue(Math.random() > 0.5, 32, refs)
    function toggle(){
      boolVal1.v = !boolVal1.v
      boolVal2.v = !boolVal2.v
      boolVal3.v = !boolVal3.v
      boolVal4.v = !boolVal4.v
      boolVal5.v = !boolVal5.v
      boolVal6.v = !boolVal6.v
    }

    elm.innerHTML = `<div><div id="bfBi_AdyQbxpmGujhPyNi"><div id="nnXlXLNTzhEjOjH_mLaPH">Static</div></div><button id="VOq_FowHMBMXHXB_YL_wi">toggle</button></div>`;

    const [bfBi_AdyQbxpmGujhPyNiRef,nnXlXLNTzhEjOjH_mLaPHRef,VOq_FowHMBMXHXB_YL_wiRef] = getElmRefs(["bfBi_AdyQbxpmGujhPyNi","nnXlXLNTzhEjOjH_mLaPH","VOq_FowHMBMXHXB_YL_wi"], 7);

    let XebjWskhZx__DmYaHSonKRef, XFXHxGWxitoxuitksqcmvRef, rdHlle_QDOqISDNfWxtSLRef, hoYjNySiVoGuRkSwvEhkyRef, JhejclsYeRUtQjLurAJhYRef, VzvsqhLIqDZW_OaoRKVcZRef

    const XebjWskhZx__DmYaHSonKAnchor = insertEmpty(bfBi_AdyQbxpmGujhPyNiRef,nnXlXLNTzhEjOjH_mLaPHRef);

    const XFXHxGWxitoxuitksqcmvAnchor = insertEmpty(bfBi_AdyQbxpmGujhPyNiRef,nnXlXLNTzhEjOjH_mLaPHRef);

    const hoYjNySiVoGuRkSwvEhkyAnchor = insertEmpty(bfBi_AdyQbxpmGujhPyNiRef,null);

    const JhejclsYeRUtQjLurAJhYAnchor = insertEmpty(bfBi_AdyQbxpmGujhPyNiRef,null);

    addEvListener(VOq_FowHMBMXHXB_YL_wiRef, "click", toggle);

    const renderXebjWskhZx__DmYaHSonKElm = () => {
        XebjWskhZx__DmYaHSonKRef = document.createElement("div");

        XebjWskhZx__DmYaHSonKRef.innerHTML = "AAA";

        bfBi_AdyQbxpmGujhPyNiRef.insertBefore(XebjWskhZx__DmYaHSonKRef, XebjWskhZx__DmYaHSonKAnchor);

    }

    boolVal1.v && renderXebjWskhZx__DmYaHSonKElm()

    const renderXFXHxGWxitoxuitksqcmvElm = () => {
        XFXHxGWxitoxuitksqcmvRef = document.createElement("div");

        XFXHxGWxitoxuitksqcmvRef.innerHTML = "BBB";

        bfBi_AdyQbxpmGujhPyNiRef.insertBefore(XFXHxGWxitoxuitksqcmvRef, XFXHxGWxitoxuitksqcmvAnchor);

    }

    boolVal2.v && renderXFXHxGWxitoxuitksqcmvElm()

    const renderrdHlle_QDOqISDNfWxtSLElm = () => {
        rdHlle_QDOqISDNfWxtSLRef = document.createElement("div");

        rdHlle_QDOqISDNfWxtSLRef.innerHTML = "CCC";

        bfBi_AdyQbxpmGujhPyNiRef.insertBefore(rdHlle_QDOqISDNfWxtSLRef, nnXlXLNTzhEjOjH_mLaPHRef);

    }

    boolVal3.v && renderrdHlle_QDOqISDNfWxtSLElm()

    const renderhoYjNySiVoGuRkSwvEhkyElm = () => {
        hoYjNySiVoGuRkSwvEhkyRef = document.createElement("div");

        hoYjNySiVoGuRkSwvEhkyRef.innerHTML = "DDD";

        bfBi_AdyQbxpmGujhPyNiRef.insertBefore(hoYjNySiVoGuRkSwvEhkyRef, hoYjNySiVoGuRkSwvEhkyAnchor);

    }

    boolVal4.v && renderhoYjNySiVoGuRkSwvEhkyElm()

    const renderJhejclsYeRUtQjLurAJhYElm = () => {
        JhejclsYeRUtQjLurAJhYRef = document.createElement("div");

        JhejclsYeRUtQjLurAJhYRef.innerHTML = "EEE";

        bfBi_AdyQbxpmGujhPyNiRef.insertBefore(JhejclsYeRUtQjLurAJhYRef, JhejclsYeRUtQjLurAJhYAnchor);

    }

    boolVal5.v && renderJhejclsYeRUtQjLurAJhYElm()

    const renderVzvsqhLIqDZW_OaoRKVcZElm = () => {
        VzvsqhLIqDZW_OaoRKVcZRef = document.createElement("div");

        VzvsqhLIqDZW_OaoRKVcZRef.innerHTML = "FFF";

        bfBi_AdyQbxpmGujhPyNiRef.insertBefore(VzvsqhLIqDZW_OaoRKVcZRef, null);

    }

    boolVal6.v && renderVzvsqhLIqDZW_OaoRKVcZElm()

    refs[2] = genUpdateFunc(() => {
        refs[0] & 1 && boolVal1.v ? renderXebjWskhZx__DmYaHSonKElm() : XebjWskhZx__DmYaHSonKRef.remove()

        refs[0] & 2 && boolVal2.v ? renderXFXHxGWxitoxuitksqcmvElm() : XFXHxGWxitoxuitksqcmvRef.remove()

        refs[0] & 4 && boolVal3.v ? renderrdHlle_QDOqISDNfWxtSLElm() : rdHlle_QDOqISDNfWxtSLRef.remove()

        refs[0] & 8 && boolVal4.v ? renderhoYjNySiVoGuRkSwvEhkyElm() : hoYjNySiVoGuRkSwvEhkyRef.remove()

        refs[0] & 16 && boolVal5.v ? renderJhejclsYeRUtQjLurAJhYElm() : JhejclsYeRUtQjLurAJhYRef.remove()

        refs[0] & 32 && boolVal6.v ? renderVzvsqhLIqDZW_OaoRKVcZElm() : VzvsqhLIqDZW_OaoRKVcZRef.remove()

    });

}