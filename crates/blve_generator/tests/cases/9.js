
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let showBlock = reactiveValue(true, 1, refs)
    function toggle() {
      showBlock.v = !showBlock.v
    }

    elm.innerHTML = `<div id="tkImxPylvz_lkHpfAnQdb" />`;

    const [tkImxPylvz_lkHpfAnQdbRef] = getElmRefs(["tkImxPylvz_lkHpfAnQdb"], 1);

    let OlNdvhscaQqrghWIlCAWeRef

    const renderOlNdvhscaQqrghWIlCAWeElm = () => {
        OlNdvhscaQqrghWIlCAWeRef = document.createElement("div");

        OlNdvhscaQqrghWIlCAWeRef.innerHTML = `
            THIS IS IF BLOCK1
          `;

        tkImxPylvz_lkHpfAnQdbRef.insertBefore(OlNdvhscaQqrghWIlCAWeRef, null);

    }

    showBlock.v && renderOlNdvhscaQqrghWIlCAWeElm()

    refs[2] = genUpdateFunc(() => {
        refs[0] & 1 && showBlock.v ? renderOlNdvhscaQqrghWIlCAWeElm() : OlNdvhscaQqrghWIlCAWeRef.remove()

    });

}