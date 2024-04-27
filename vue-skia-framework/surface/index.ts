import {
    h,
    ref,
    onMounted,
    onBeforeUnmount,
    onUpdated,
    getCurrentInstance,
    VNodeProps,
    SetupContext
} from 'vue';
import { ComponentInternalInstanceWithSoftSkiaWASM } from "../type";
import { SelfIncreaseCount } from "../common"

export default {
    props: {
        config: {
            type: Object,
            default: function () {
                return {};
            },
        },
        __useStrictMode: {
            type: Boolean,
        },
    },

    inheritAttrs: false,

    setup(_props: VNodeProps, { attrs, slots }: SetupContext) {

        const container = ref<HTMLImageElement>(null);
        const instance = getCurrentInstance() as ComponentInternalInstanceWithSoftSkiaWASM;
        const ssw = window.ssw;
        const rootID = SelfIncreaseCount.count;
        const core = new ssw.SoftSkiaWASM(rootID);

        let waitingForDraw = false;

        instance.ssw = core; // Save on component instance
        instance._ssw_id = rootID;
        core.setAttrBySerde(rootID, { attr: { R: { x: 0, y: 0, width: attrs.width, height: attrs.height, color: 'transparent', style: "fill" } } })

        // batch draw func
        function batchDraw() {
            if (!waitingForDraw) {
                waitingForDraw = true;
                window.requestAnimationFrame(() => {
                    const base64 = core.toBase64();
                    container.value.setAttribute("src", base64);
                    waitingForDraw = false;
                });
            }
        }
        instance._ssw_batchDraw = () => batchDraw();

        onMounted(() => {
            batchDraw();
        });

        onUpdated(() => {
            batchDraw();
        });

        onBeforeUnmount(() => {});


        return () => h('img', { ref: container }, slots.default?.());
    }
}