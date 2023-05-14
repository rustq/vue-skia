const {
    h,
    withDirectives,
    ref,
    watch,
    onMounted,
    onBeforeUnmount,
    onUpdated,
    getCurrentInstance,
    reactive,
} = require('vue');

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

    setup(props: any, {attrs, slots, expose, emits}: any) {

        const container = ref(null);
        const instance = getCurrentInstance();
        // @ts-ignore
        const ssw = global.ssw;
        const core = new ssw.SoftSkiaWASM();
        instance.ssw = core; // Save on component instance
        console.log(core)
        core.setShapeToChild(0, 0, 0, attrs.width, attrs.height, 0, 0, 0, 0)


        onMounted(() => {
            const base64 = core.toBase64();
            container.value.setAttribute("src", base64);
        });

        onUpdated(() => {
            const base64 = core.toBase64();
            console.log(core.toDebug?.())
            let buffer = new Uint8Array(10);
            buffer.set([1,2,3,4])
            console.log(core.setShapeToChildByStream(buffer))
            const a = core.send_example_to_js();
            console.log(a)
            a.field2.push([999]);
            console.log(a.attr.R.width)
            a.attr.R.width++;
            console.log(a.attr.R.width)
            console.log(core.receive_example_from_js(a))
            container.value.setAttribute("src", base64);
        });

        onBeforeUnmount(() => {
        });


        return () => h('img', { ref: container }, slots.default?.());
    }
}