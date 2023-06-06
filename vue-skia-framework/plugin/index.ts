import {
    h,
    onMounted,
    onBeforeUnmount,
    onUpdated,
    getCurrentInstance,
    App,
} from 'vue';
import Surface from "../surface";
import { ComponentInternalInstanceWithSoftSkiaWASM } from "../type";

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
                const instance = getCurrentInstance() as ComponentInternalInstanceWithSoftSkiaWASM;
                var root = instance as ComponentInternalInstanceWithSoftSkiaWASM;
                while (!root.ssw) {
                    root = root.parent as ComponentInternalInstanceWithSoftSkiaWASM;
                }
                const core = root.ssw;
                instance._ssw_id = SELF_INCREASE_COUNT();
                const next = instance.vnode.el.nextElementSibling?.__vnode.ctx._ssw_id;
                if (next) {
                    core.createChildInsertBeforeElementOfContainer(instance._ssw_id, next, 0)
                } else {
                    core.createChildAppendToContainer(instance._ssw_id, 0);
                }
                updateInstance(name, instance, attrs);
            });

            onUpdated(() => {
                const instance = getCurrentInstance() as ComponentInternalInstanceWithSoftSkiaWASM;
                var root = instance as ComponentInternalInstanceWithSoftSkiaWASM;
                while (!root.ssw) {
                    root = root.parent as ComponentInternalInstanceWithSoftSkiaWASM;
                }
                const core = root.ssw;
                updateInstance(name, instance, attrs);
            });

            /**
             * updateInstance
             * @param name 
             * @param instance 
             * @param attrs 
             */
            function updateInstance(name: string, instance: ComponentInternalInstanceWithSoftSkiaWASM, attrs: any) {
                if (name === 'Rect') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { R: { x: attrs.x, y: attrs.y, width: attrs.width, height: attrs.height, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'Circle') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { C: { cx: attrs.cx, cy: attrs.cy, r: attrs.r, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'RoundRect') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { RR: { x: attrs.x, y: attrs.y, r: attrs.r, width: attrs.width, height: attrs.height, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'Line') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { L: { p1: attrs.p1, p2: attrs.p2, stroke_width: attrs.strokeWidth, color: attrs.color } } })
                }
                if (name === 'Points') {
                    core.setShapeBySerde(instance._ssw_id, { attr: { P: { points: attrs.points, stroke_width: attrs.strokeWidth, color: attrs.color, style: attrs.style } } })
                }
            }

            onBeforeUnmount(() => {
                const instance = getCurrentInstance() as ComponentInternalInstanceWithSoftSkiaWASM;
                var root = instance as ComponentInternalInstanceWithSoftSkiaWASM;
                while (!root.ssw) {
                    root = root.parent as ComponentInternalInstanceWithSoftSkiaWASM;
                }
                const core = root.ssw;
                const child_id = instance._ssw_id;
                core.removeChildFromContainer(child_id, (instance.parent as ComponentInternalInstanceWithSoftSkiaWASM)._ssw_id)
            });

            return () => h(name, {}, slots.default?.())
        }
    }
}

const VueSkiaPlugin = {
    install: (app: App) => {
        let prefixToUse = 'vSk';
        WidgetList.forEach((name) => {
            app.component(`${prefixToUse}${name}`, VSKNode(name));
        });
    },
} as { install: (app: App<any>) => any; };

export default VueSkiaPlugin;
