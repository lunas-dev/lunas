
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let boolVal1 = reactiveValue(false, 1, refs)
    function toggle() {
      boolVal1.v = !boolVal1.v
    }

    elm.innerHTML = `<div id="_forhNDTXhMiXqfFWTKxt"><div id="WUpPtHUBjCrfqyDwDhGzL">AAA</div><button id="EDZocVZQN$zjsPZ$qPZbm">toggle</button><div id="AWeMABHBqZCuzpFfcRcCa">${escapeHtml(boolVal1.v)}</div></div>`;

    const [EDZocVZQN$zjsPZ$qPZbmRef,AWeMABHBqZCuzpFfcRcCaRef,_forhNDTXhMiXqfFWTKxtRef,WUpPtHUBjCrfqyDwDhGzLRef] = getElmRefs(["EDZocVZQN$zjsPZ$qPZbm","AWeMABHBqZCuzpFfcRcCa","_forhNDTXhMiXqfFWTKxt","WUpPtHUBjCrfqyDwDhGzL"], 15);

    let GamcKWqwrHXbxxZAvKJHHRef

    addEvListener(EDZocVZQN$zjsPZ$qPZbmRef, "click", toggle);

    const renderGamcKWqwrHXbxxZAvKJHHElm = () => {
        GamcKWqwrHXbxxZAvKJHHRef = document.createElement("div");

        GamcKWqwrHXbxxZAvKJHHRef.innerHTML = "AAA";

        _forhNDTXhMiXqfFWTKxtRef.insertBefore(GamcKWqwrHXbxxZAvKJHHRef, WUpPtHUBjCrfqyDwDhGzLRef);

    }

    boolVal1.v && renderGamcKWqwrHXbxxZAvKJHHRef()

    refs[2] = genUpdateFunc(() => {
        refs[0] & 1 && replaceText(`${escapeHtml(boolVal1.v)}`, AWeMABHBqZCuzpFfcRcCaRef);

        refs[0] & 1 && boolVal1.v ? renderGamcKWqwrHXbxxZAvKJHHElm() : GamcKWqwrHXbxxZAvKJHHRef.remove()

    });

}