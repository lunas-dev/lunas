
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let inputValue = reactiveValue("", 1, refs)
    function addFoo() {
      inputValue.v += "foo"
    }

    elm.innerHTML = `<div></div>`;

    const [] = getElmRefs([], 0);

    refs[2] = genUpdateFunc(() => {
    
    });

}