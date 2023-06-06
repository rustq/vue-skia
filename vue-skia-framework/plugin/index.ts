import {
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
} from 'vue';
import Surface from "../surface";

let _SELF_INCREASE_COUNT = 1;
let SELF_INCREASE_COUNT = () => _SELF_INCREASE_COUNT++

const WidgetList = [
    'Surface',
    'Points',
    'Line',
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
                // @ts-expect-error
                while (!root.ssw) {
                    root = root.parent;
                }
                // @ts-expect-error
                const core = root.ssw;
                // @ts-expect-error
                instance._ssw_id = SELF_INCREASE_COUNT();
                const next = instance.vnode.el.nextElementSibling?.__vnode.ctx._ssw_id;
                if (next) {
                    // @ts-expect-error
                    core.createChildInsertBeforeElementOfContainer(instance._ssw_id, next, 0)
                } else {
                    // @ts-expect-error
                    core.createChildAppendToContainer(instance._ssw_id, 0);
                }
                if (name === 'Rect') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { R: { x: attrs.x, y: attrs.y, width: attrs.width, height: attrs.height, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'Circle') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { C: { cx: attrs.cx, cy: attrs.cy, r: attrs.r, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'RoundRect') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { RR: { x: attrs.x, y: attrs.y, r: attrs.r, width: attrs.width, height: attrs.height, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'Line') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { L: { p1: attrs.p1, p2: attrs.p2, stroke_width: attrs.strokeWidth, color: attrs.color } } })
                }
                if (name === 'Points') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { P: { points: attrs.points, stroke_width: attrs.strokeWidth, color: attrs.color, style: attrs.style } } })
                }
            });

            onUpdated(() => {
                const instance = getCurrentInstance();
                var root = instance;
                // @ts-expect-error
                while (!root.ssw) {
                    root = root.parent;
                }
                // @ts-expect-error
                const core = root.ssw;
                if (name === 'Rect') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { R: { x: attrs.x, y: attrs.y, width: attrs.width, height: attrs.height, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'Circle') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { C: { cx: attrs.cx, cy: attrs.cy, r: attrs.r, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'RoundRect') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { RR: { x: attrs.x, y: attrs.y, r: attrs.r, width: attrs.width, height: attrs.height, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'Line') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { L: { p1: attrs.p1, p2: attrs.p2, stroke_width: attrs.strokeWidth, color: attrs.color } } })
                }
                if (name === 'Points') {
                    // @ts-expect-error
                    core.setShapeBySerde(instance._ssw_id, { attr: { P: { points: attrs.points, stroke_width: attrs.strokeWidth, color: attrs.color, style: attrs.style } } })
                }
            });

            onBeforeUnmount(() => {
                const instance = getCurrentInstance();
                var root = instance;
                // @ts-expect-error
                while (!root.ssw) {
                    root = root.parent;
                }
                // @ts-expect-error
                const core = root.ssw;
                // @ts-expect-error
                const child_id = instance._ssw_id;
                // @ts-expect-error
                core.removeChildFromContainer(child_id, instance.parent._ssw_id)
            });

            return () => h(name, {}, slots.default?.())
        }
    }
}

const VueSkiaPlugin = {
    install: (app: any) => {
        let prefixToUse = 'v';
        WidgetList.forEach((name) => {
            app.component(`${prefixToUse}${name}`, VSKNode(name));
        });
    },
};

export default VueSkiaPlugin;
