
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    const balue = reactiveValue("", 1, refs)
    function onInput(e) {
      balue.v = e.target.value
    }
    function xxx() {
      balue.v += "xxx"
    }

    elm.innerHTML = `<div></div>`;

    const [] = getElmRefs([], 0);

    refs[2] = genUpdateFunc(() => {
    
    });

}