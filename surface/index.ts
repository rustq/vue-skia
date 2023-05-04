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
        const v_sk = window.v_sk;
        const core = new v_sk.Container();
        instance.__vsk_core = core; // Save on component instance
        console.log('setup Surface', v_sk, instance)
        core.setXYWHBByNodeID(1, 0, 0, 400, 400, 255, 0, 0, 100)


        onMounted(() => {
            //
            // @ts-ignore
            console.log('mounted surface');
            console.log(core.debug())
            console.log(core.renderRootToStream())

            const base64 = core.renderRootToBase64();
            console.log(container.value)
            container.value.setAttribute("src", base64);
        });

        onUpdated(() => {
            const base64 = core.renderRootToBase64();
            console.log(container.value)
            container.value.setAttribute("src", base64);
        });

        onBeforeUnmount(() => {
            console.log('surface destroy')
        });


        return () => h('img', { ref: container }, slots.default?.());
    }
}