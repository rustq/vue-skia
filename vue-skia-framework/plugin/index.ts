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
    getCurrentScope,
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
        setup(props: any, { attrs, slots, expose, ...other }: any) {

            onMounted(() => {
                const instance = getCurrentInstance();
                var root = instance;
                while (!root.ssw) {
                    root = root.parent;
                }
                const core = root.ssw;
                instance._ssw_id = SELF_INCREASE_COUNT();
                const next = instance.vnode.el.nextElementSibling?.__vnode.ctx._ssw_id;
                if (next) {
                    core.createChildInsertBeforeElementOfContainer(instance._ssw_id, next, 0)
                } else {
                    core.createChildAppendToContainer(instance._ssw_id, 0);
                }
                if (name === 'Rect') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { R: { x: attrs.x, y: attrs.y, width: attrs.width, height: attrs.height, color: [attrs.r, attrs.g, attrs.b, attrs.a] } } })
                }
                if (name === 'Circle') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { C: { cx: attrs.cx, cy: attrs.cy, r: attrs.r, color: [attrs.r, attrs.g, attrs.b, attrs.a] } } })
                }
                if (name === 'RoundRect') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { RR: { x: attrs.x, y: attrs.y, r: attrs.r, width: attrs.width, height: attrs.height, color: [attrs.r, attrs.g, attrs.b, attrs.a] } } })
                }
            });

            onUpdated(() => {
                const instance = getCurrentInstance();
                var root = instance;
                while (!root.ssw) {
                    root = root.parent;
                }
                const core = root.ssw;
                if (name === 'Rect') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { R: { x: attrs.x, y: attrs.y, width: attrs.width, height: attrs.height, color: [attrs.r, attrs.g, attrs.b, attrs.a] } } })
                }
                if (name === 'Circle') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { C: { cx: attrs.cx, cy: attrs.cy, r: attrs.r, color: [attrs.r, attrs.g, attrs.b, attrs.a] } } })
                }
            });

            onBeforeUnmount(() => {
                const instance = getCurrentInstance();
                var root = instance;
                while (!root.ssw) {
                    root = root.parent;
                }
                const core = root.ssw;
                const child_id = instance._ssw_id;
                core.removeChildFromContainer(child_id, instance.parent._ssw_id)
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
