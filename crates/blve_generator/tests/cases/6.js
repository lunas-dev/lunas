
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let boolVal1 = reactiveValue(false, 1, refs)
    let boolVal2 = reactiveValue(false, 2, refs)
    let boolVal3 = reactiveValue(false, 4, refs)
    let boolVal4 = reactiveValue(false, 8, refs)
    let boolVal5 = reactiveValue(false, 16, refs)
    let boolVal6 = reactiveValue(false, 32, refs)

    elm.innerHTML = `<div id="j_ZjIsPUytBwiDORmGkpq"><div id="ueDjp_xkuYrIQADTKnNQh">AAA</div></div>`;

    const [j_ZjIsPUytBwiDORmGkpqRef,ueDjp_xkuYrIQADTKnNQhRef] = getElmRefs(["j_ZjIsPUytBwiDORmGkpq","ueDjp_xkuYrIQADTKnNQh"], 3);

    let xiwrLHNUBjfkDxkfdIQ$NRef, UpWwfinXGojyvLQDmgwQCRef, oGwPtxGItNWUyA$$KJZElRef, YPh$qJ_RSkVmLFgPMMbiBRef, DzPSZCXJdOHevZtlQpt$zRef, tCkBhMlGzZGJPAQYuYWBURef

    const xiwrLHNUBjfkDxkfdIQ$NAnchor = insertEmpty(j_ZjIsPUytBwiDORmGkpqRef,ueDjp_xkuYrIQADTKnNQhRef);

    const UpWwfinXGojyvLQDmgwQCAnchor = insertEmpty(j_ZjIsPUytBwiDORmGkpqRef,ueDjp_xkuYrIQADTKnNQhRef);

    const YPh$qJ_RSkVmLFgPMMbiBAnchor = insertEmpty(j_ZjIsPUytBwiDORmGkpqRef,null);

    const DzPSZCXJdOHevZtlQpt$zAnchor = insertEmpty(j_ZjIsPUytBwiDORmGkpqRef,null);

    const renderxiwrLHNUBjfkDxkfdIQ$NElm = () => {
        xiwrLHNUBjfkDxkfdIQ$NRef = document.createElement("div");

        xiwrLHNUBjfkDxkfdIQ$NRef.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(xiwrLHNUBjfkDxkfdIQ$NRef, xiwrLHNUBjfkDxkfdIQ$NAnchor);

    }

    boolVal1.v && renderxiwrLHNUBjfkDxkfdIQ$NElm()

    const renderUpWwfinXGojyvLQDmgwQCElm = () => {
        UpWwfinXGojyvLQDmgwQCRef = document.createElement("div");

        UpWwfinXGojyvLQDmgwQCRef.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(UpWwfinXGojyvLQDmgwQCRef, UpWwfinXGojyvLQDmgwQCAnchor);

    }

    boolVal2.v && renderUpWwfinXGojyvLQDmgwQCElm()

    const renderoGwPtxGItNWUyA$$KJZElElm = () => {
        oGwPtxGItNWUyA$$KJZElRef = document.createElement("div");

        oGwPtxGItNWUyA$$KJZElRef.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(oGwPtxGItNWUyA$$KJZElRef, ueDjp_xkuYrIQADTKnNQhRef);

    }

    boolVal3.v && renderoGwPtxGItNWUyA$$KJZElElm()

    const renderYPh$qJ_RSkVmLFgPMMbiBElm = () => {
        YPh$qJ_RSkVmLFgPMMbiBRef = document.createElement("div");

        YPh$qJ_RSkVmLFgPMMbiBRef.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(YPh$qJ_RSkVmLFgPMMbiBRef, YPh$qJ_RSkVmLFgPMMbiBAnchor);

    }

    boolVal4.v && renderYPh$qJ_RSkVmLFgPMMbiBElm()

    const renderDzPSZCXJdOHevZtlQpt$zElm = () => {
        DzPSZCXJdOHevZtlQpt$zRef = document.createElement("div");

        DzPSZCXJdOHevZtlQpt$zRef.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(DzPSZCXJdOHevZtlQpt$zRef, DzPSZCXJdOHevZtlQpt$zAnchor);

    }

    boolVal5.v && renderDzPSZCXJdOHevZtlQpt$zElm()

    const rendertCkBhMlGzZGJPAQYuYWBUElm = () => {
        tCkBhMlGzZGJPAQYuYWBURef = document.createElement("div");

        tCkBhMlGzZGJPAQYuYWBURef.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(tCkBhMlGzZGJPAQYuYWBURef, null);

    }

    boolVal6.v && rendertCkBhMlGzZGJPAQYuYWBUElm()

    refs[2] = genUpdateFunc(() => {
        refs[0] & 1 && boolVal1.v ? renderxiwrLHNUBjfkDxkfdIQ$NElm() : xiwrLHNUBjfkDxkfdIQ$NRef.remove()

        refs[0] & 2 && boolVal2.v ? renderUpWwfinXGojyvLQDmgwQCElm() : UpWwfinXGojyvLQDmgwQCRef.remove()

        refs[0] & 4 && boolVal3.v ? renderoGwPtxGItNWUyA$$KJZElElm() : oGwPtxGItNWUyA$$KJZElRef.remove()

        refs[0] & 8 && boolVal4.v ? renderYPh$qJ_RSkVmLFgPMMbiBElm() : YPh$qJ_RSkVmLFgPMMbiBRef.remove()

        refs[0] & 16 && boolVal5.v ? renderDzPSZCXJdOHevZtlQpt$zElm() : DzPSZCXJdOHevZtlQpt$zRef.remove()

        refs[0] & 32 && boolVal6.v ? rendertCkBhMlGzZGJPAQYuYWBUElm() : tCkBhMlGzZGJPAQYuYWBURef.remove()

    });

}