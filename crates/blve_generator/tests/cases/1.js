
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let count = reactiveValue(0, 1, refs)
    function increment(){
      count.v++
      console.log(count.v)
    }
    function clear(){
      if(interval.v){
        clearInterval(interval.v)
        interval.v = null
      }else{
        interval.v = setInterval(increment, 2000)
      }
    }
    let interval = reactiveValue(setInterval(increment, 2000), 2, refs)

    elm.innerHTML = `<div></div>`;

    const [] = getElmRefs([], 0);

    refs[2] = genUpdateFunc(() => {
    
    });

}