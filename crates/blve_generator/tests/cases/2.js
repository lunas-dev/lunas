
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let color = reactiveValue('red', 1, refs)
    function yellow(){
      color.v = 'yellow'
    }
    function red(){
      color.v = 'red'
    }
    function blve(){
      color.v = 'blue'
    }

    elm.innerHTML = `<div></div>`;

    const [] = getElmRefs([], 0);

    refs[2] = genUpdateFunc(() => {
    
    });

}