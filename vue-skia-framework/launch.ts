const SSWInitialHelper = {
    initialStatus: 0, // 0=default 1=pending 2=succeed
    initialSucceedCallbackQueue: [] as Function[]
};

const launch = function () {
    return new Promise((resolve, _) => {
        if (SSWInitialHelper.initialStatus === 0) {
            SSWInitialHelper.initialStatus = 1;
            const wasm = import("soft-skia-wasm/soft_skia_wasm.js");
            wasm.then((ssw) => {
                ssw.default().then(() => {
                    window.ssw = ssw;
                    while (SSWInitialHelper.initialSucceedCallbackQueue.length) {
                        SSWInitialHelper.initialSucceedCallbackQueue.pop()();
                    }
                    resolve(void 0)
                })
            })
        } else if (SSWInitialHelper.initialStatus === 1) {
            SSWInitialHelper.initialSucceedCallbackQueue.push(() => resolve(void 0));
        } else if (SSWInitialHelper.initialStatus === 2) {
            resolve(void 0)
        }
    });
}

export default launch;