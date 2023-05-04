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
import Surface from "../surface";

const WidgetList = [
    'Surface',
    'Layer',
    'Triangle',
    'RoundRect',
    'Rect',
    'Circle',
];

const VSKNode = (name: string) => {
    if (name === 'Surface') {
        return Surface
    }
    return {
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
        setup(props: any, { attrs, slots, expose }: any) {
            console.log('setup', name, props)

            onMounted(() => {
                const instance = getCurrentInstance();
                var root = instance;
                while (!root.__vsk_core) {
                    root = root.parent;
                }
                const core = root.__vsk_core;

                const child_id = core.createNodeOnParentReturnNodeID(1);
                instance.__child_id = child_id;
                core.setXYWHBByNodeID(child_id, attrs.x, attrs.y, attrs.width, attrs.height, attrs.a, attrs.r, attrs.g, attrs.b);
            });

            onUpdated(() => {
                // const base64 = core.renderRootToBase64();
                // console.log(container.value)
                // container.value.setAttribute("src", base64);
                const instance = getCurrentInstance();
                var root = instance;
                while (!root.__vsk_core) {
                    root = root.parent;
                }
                const core = root.__vsk_core;
                const child_id = instance.__child_id;
                core.setXYWHBByNodeID(child_id, attrs.x, attrs.y, attrs.width, attrs.height, attrs.a, attrs.r, attrs.g, attrs.b);
            });

            onBeforeUnmount(() => {
                const instance = getCurrentInstance();
                var root = instance;
                while (!root.__vsk_core) {
                    root = root.parent;
                }
                const core = root.__vsk_core;
                const child_id = instance.__child_id;
                core.removeNodeByNodeID(child_id)
                console.log('remove', child_id)
            });

            return () => h(name, {}, slots.default?.())
        }
    }
}

const VueSkiaPlugin = {
    install: (app: any) => {
        let prefixToUse = 'v';
        // if (options && options.prefix) {
        //     prefixToUse = options.prefix;
        // }
        WidgetList.forEach((name) => {
            app.component(`${prefixToUse}${name}`, VSKNode(name));
        });
    },
};

export default VueSkiaPlugin;
