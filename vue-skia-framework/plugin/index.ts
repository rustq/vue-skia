import {
    h,
    onBeforeMount,
    onBeforeUnmount,
    onUpdated,
    getCurrentInstance,
    App,
    VNodeProps,
    SetupContext,
    Plugin
} from 'vue';
import Surface from "../surface";
import { ComponentInternalInstanceWithSoftSkiaWASM } from "../type";
import { SelfIncreaseCount } from "../common"
import { SoftSkiaWASM } from '../../soft-skia-wasm/pkg/soft_skia_wasm';

const WidgetList = [
    'Surface',
    'Group',
    'GroupClip',
    'Points',
    'Line',
    'RoundRect',
    'Rect',
    'Circle',
    'Image',
    'Text' 
];

const VSKNode = (name: string) => {
    if (name === 'Surface') {
        return Surface
    }

    if (name === "GroupClip") {
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
        setup(_props: VNodeProps, { slots }: SetupContext) {
          onBeforeMount(() => {
            const instance =
              getCurrentInstance() as ComponentInternalInstanceWithSoftSkiaWASM;
            var root = instance as ComponentInternalInstanceWithSoftSkiaWASM;
            while (!root.ssw) {
              root = root.parent as ComponentInternalInstanceWithSoftSkiaWASM;
            }
            const core = root.ssw;

            let parent = instance.parent;

            while (!("_ssw_id" in parent)) {
              parent = parent.parent;
            }

            instance._ssw_id = (
              parent as ComponentInternalInstanceWithSoftSkiaWASM
            )._ssw_id;

            instance._ssw_grouped = (child) => {
              core.setAttrBySerde(instance._ssw_id, {
                attr: {
                  GC: {
                    clip: child._ssw_id,
                  },
                },
              });
            };
          });

          return () => h(name, {}, slots.default?.());
        },
      };
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
        setup(_props: VNodeProps, { attrs, slots }: SetupContext) {

            onBeforeMount(() => {
                const instance = getCurrentInstance() as ComponentInternalInstanceWithSoftSkiaWASM;
                var root = instance as ComponentInternalInstanceWithSoftSkiaWASM;
                while (!root.ssw) {
                    root = root.parent as ComponentInternalInstanceWithSoftSkiaWASM;
                }
                const core = root.ssw;
                instance._ssw_id = SelfIncreaseCount.count;
                var parent = instance.parent;
                while (!('_ssw_id' in parent)) {
                    parent = parent.parent;
                }

                core.createChildAppendToContainer(instance._ssw_id, (parent as ComponentInternalInstanceWithSoftSkiaWASM)._ssw_id);
                updateInstance(core, name, instance, attrs);
                (
                  parent as ComponentInternalInstanceWithSoftSkiaWASM
                )._ssw_grouped?.(instance);
            });

            onUpdated(() => {
                const instance = getCurrentInstance() as ComponentInternalInstanceWithSoftSkiaWASM;
                var root = instance as ComponentInternalInstanceWithSoftSkiaWASM;
                while (!root.ssw) {
                    root = root.parent as ComponentInternalInstanceWithSoftSkiaWASM;
                }
                const core = root.ssw;
                updateInstance(core, name, instance, attrs);
            });

            /**
             * updateInstance
             * @param name 
             * @param instance 
             * @param attrs 
             */
            function updateInstance(core: SoftSkiaWASM, name: string, instance: ComponentInternalInstanceWithSoftSkiaWASM, attrs: SetupContext['attrs']) {
                if (name === 'Rect') {
                    core.setAttrBySerde(instance._ssw_id, { attr: { R: { x: attrs.x, y: attrs.y, width: attrs.width, height: attrs.height, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'Circle') {
                    core.setAttrBySerde(instance._ssw_id, { attr: { C: { cx: attrs.cx, cy: attrs.cy, r: attrs.r, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'RoundRect') {
                    core.setAttrBySerde(instance._ssw_id, { attr: { RR: { x: attrs.x, y: attrs.y, r: attrs.r, width: attrs.width, height: attrs.height, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'Line') {
                    core.setAttrBySerde(instance._ssw_id, { attr: { L: { p1: attrs.p1, p2: attrs.p2, stroke_width: attrs.strokeWidth, color: attrs.color } } })
                }
                if (name === 'Points') {
                    core.setAttrBySerde(instance._ssw_id, { attr: { P: { points: attrs.points, stroke_width: attrs.strokeWidth, color: attrs.color, style: attrs.style } } })
                }
                if (name === 'Group') {
                    core.setAttrBySerde(instance._ssw_id, {
                      attr: {
                        G: {
                          x: attrs.x,
                          y: attrs.y,
                          color: attrs.color,
                          stroke_width: attrs.strokeWidth,
                          style: attrs.style,
                          invert_clip: attrs.invertClip,
                        },
                      },
                    });
                }
                if (name === "Image") {
                  core.setAttrBySerde(instance._ssw_id, {
                    attr: {
                      I: {
                        image: attrs.image,
                        x: attrs.x,
                        y: attrs.y,
                        width: attrs.width,
                        height: attrs.height,
                      },
                    },
                  });
                }
                if (name === "Text") {
                  core.setAttrBySerde(instance._ssw_id, {
                    attr: {
                      T: {
                        text: attrs.text,
                        x: attrs.x,
                        y: attrs.y,
                        font_size: attrs.fontSize,
                        color: attrs.color,
                      },
                    },
                  });
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
                var parent = instance.parent;
                while (!('_ssw_id' in parent)) {
                    parent = parent.parent;
                }
                core.removeChildFromContainer(child_id, (parent as ComponentInternalInstanceWithSoftSkiaWASM)._ssw_id)
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
} as Plugin;

export default VueSkiaPlugin;
