
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

    const xiwrLHNUBjfkDxkfdIQ$NAnchor = insertEmpty(j_ZjIsPUytBwiDORmGkpqRef,ueDjp_xkuYrIQADTKnNQhRef);

    const UpWwfinXGojyvLQDmgwQCAnchor = insertEmpty(j_ZjIsPUytBwiDORmGkpqRef,ueDjp_xkuYrIQADTKnNQhRef);

    const YPh$qJ_RSkVmLFgPMMbiBAnchor = insertEmpty(j_ZjIsPUytBwiDORmGkpqRef,null);

    const DzPSZCXJdOHevZtlQpt$zAnchor = insertEmpty(j_ZjIsPUytBwiDORmGkpqRef,null);

    const renderxiwrLHNUBjfkDxkfdIQ$NElm = () => {
        let xiwrLHNUBjfkDxkfdIQ$NElm = document.createElement("div");

        xiwrLHNUBjfkDxkfdIQ$NElm.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(xiwrLHNUBjfkDxkfdIQ$NElm, xiwrLHNUBjfkDxkfdIQ$NAnchor);

    }

    boolVal1.v && renderxiwrLHNUBjfkDxkfdIQ$NElm()

    const renderUpWwfinXGojyvLQDmgwQCElm = () => {
        let UpWwfinXGojyvLQDmgwQCElm = document.createElement("div");

        UpWwfinXGojyvLQDmgwQCElm.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(UpWwfinXGojyvLQDmgwQCElm, UpWwfinXGojyvLQDmgwQCAnchor);

    }

    boolVal2.v && renderUpWwfinXGojyvLQDmgwQCElm()

    const renderoGwPtxGItNWUyA$$KJZElElm = () => {
        let oGwPtxGItNWUyA$$KJZElElm = document.createElement("div");

        oGwPtxGItNWUyA$$KJZElElm.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(oGwPtxGItNWUyA$$KJZElElm, ueDjp_xkuYrIQADTKnNQhRef);

    }

    boolVal3.v && renderoGwPtxGItNWUyA$$KJZElElm()

    const renderYPh$qJ_RSkVmLFgPMMbiBElm = () => {
        let YPh$qJ_RSkVmLFgPMMbiBElm = document.createElement("div");

        YPh$qJ_RSkVmLFgPMMbiBElm.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(YPh$qJ_RSkVmLFgPMMbiBElm, YPh$qJ_RSkVmLFgPMMbiBAnchor);

    }

    boolVal4.v && renderYPh$qJ_RSkVmLFgPMMbiBElm()

    const renderDzPSZCXJdOHevZtlQpt$zElm = () => {
        let DzPSZCXJdOHevZtlQpt$zElm = document.createElement("div");

        DzPSZCXJdOHevZtlQpt$zElm.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(DzPSZCXJdOHevZtlQpt$zElm, DzPSZCXJdOHevZtlQpt$zAnchor);

    }

    boolVal5.v && renderDzPSZCXJdOHevZtlQpt$zElm()

    const rendertCkBhMlGzZGJPAQYuYWBUElm = () => {
        let tCkBhMlGzZGJPAQYuYWBUElm = document.createElement("div");

        tCkBhMlGzZGJPAQYuYWBUElm.innerHTML = "AAA";

        j_ZjIsPUytBwiDORmGkpqRef.insertBefore(tCkBhMlGzZGJPAQYuYWBUElm, null);

    }

    boolVal6.v && rendertCkBhMlGzZGJPAQYuYWBUElm()

    refs[2] = genUpdateFunc(() => {

    });

}