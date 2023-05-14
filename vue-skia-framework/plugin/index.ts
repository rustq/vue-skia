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

let _SELF_INCREASE_COUNT = 1;
let SELF_INCREASE_COUNT = () => _SELF_INCREASE_COUNT++

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

            onMounted(() => {
                const instance = getCurrentInstance();
                var root = instance;
                while (!root.ssw) {
                    root = root.parent;
                }
                const core = root.ssw;
                instance._ssw_id = SELF_INCREASE_COUNT();
                core.createChildAppendToContainer(instance._ssw_id, 0);
                core.setShapeBySerde(instance._ssw_id, { field2: [[0]], field3: [0,0,0,0], attr: { R: { x: attrs.x, y: attrs.y, width: attrs.width, height: attrs.height, color: [240, 230, 0, 100] } } })
                // core.setShapeToChild(instance._ssw_id, attrs.x, attrs.y, attrs.width, attrs.height, attrs.r, attrs.g, attrs.b, attrs.a);
            });

            onUpdated(() => {
                const instance = getCurrentInstance();
                var root = instance;
                while (!root.ssw) {
                    root = root.parent;
                }
                const core = root.ssw;
                core.setShapeBySerde(instance._ssw_id, { field2: [], field3: [], attr: { R: { x: attrs.x, y: attrs.y, width: attrs.width, height: attrs.height } } })
                // core.setShapeToChild(instance._ssw_id, attrs.x, attrs.y, attrs.width, attrs.height, attrs.r, attrs.g, attrs.b, attrs.a);
            });

            onBeforeUnmount(() => {
                const instance = getCurrentInstance();
                var root = instance;
                while (!root.ssw) {
                    root = root.parent;
                }
                const core = root.ssw;
                const child_id = instance._ssw_id;
                core.removeChildFromContainer(child_id, root.parent._ssw_id)
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
