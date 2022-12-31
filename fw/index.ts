import {
    h,
    ref,
    reactive,
    watch,
    onMounted,
    onUnmounted,
    onUpdated,
    getCurrentInstance,
  } from 'vue';

class Layer {
    private children = [];

    constructor() {
      console.log('Layer', this)
    }

    toString() {
        return 'Layer'
    }
}

class Rect {
    private children = [];

    toString() {
        return 'Rect'
    }
}

class Circle {
    private children = [];

    toString() {
        return 'Circle'
    }
}

const WidgetList = {
    'Layer': Layer,
    'Rect': Rect,
    'Circle': Circle,
};

const FWNode = (name) => {
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
        setup(props, { attrs, slots, expose }) {
            const instance = getCurrentInstance();
            const oldProps = reactive({});
            const NodeClass = WidgetList[name];
            const _fwNode = new NodeClass();
            // @ts-expect-error
            instance!.__fwNode = _fwNode;
            // @ts-expect-error
            instance!.vnode.__fwNode = _fwNode;
            uploadKonva();

            function getNode() {
                // @ts-expect-error
              return instance.__fwNode;
            }
            function getStage() {
                // @ts-expect-error
              return instance.__fwNode;
            }
      
            function uploadKonva() {
              const events = {};
              // @ts-expect-error
              for (var key in instance.vnode.props) {
                if (key.slice(0, 2) === 'on') {
                  // @ts-expect-error
                  events[key] = instance.vnode.props[key];
                }
              }
              const existingProps = oldProps || {};
              const newProps = {
                ...attrs,
                ...props.config,
                ...events,
              };

              Object.assign(oldProps, newProps);
            }

            onMounted(() => {
                // @ts-expect-error
                if (instance.parent?.__fwNode) {
                  // @ts-expect-error
                  instance.parent.__fwNode.children.push(_fwNode);
                }
              });

            onUpdated(() => {
                uploadKonva();
            });

            watch(() => props.config, uploadKonva, { deep: true });
            expose({
                getStage,
                getNode,
              });

            const isContainer = true;
            return isContainer
            ? () => h('template', {}, slots.default?.())
            : () => null;
        }
    }
}

const FW = {
    install: (app, options) => {
      let prefixToUse = 'v';
      if (options && options.prefix) {
        prefixToUse = options.prefix;
      }
      ['Layer', 'Rect', 'Circle'].forEach((name) => {
        app.component(`${prefixToUse}${name}`, FWNode(name));
      });
    },
  };
  
  export default FW;