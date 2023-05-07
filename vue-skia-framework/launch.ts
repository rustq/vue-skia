const launch = function () {
    return new Promise((resolve, _) => {
        const wasm = import("../soft-skia-wasm/pkg/soft_skia_wasm.js");
        wasm.then((ssw) => {
            ssw.default().then(() => {
                // @ts-ignore
                global.ssw = ssw;

                resolve(void 0)
            })
        })
    });
}

export default launch;