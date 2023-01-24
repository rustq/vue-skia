module.exports =
/******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "/";
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = 0);
/******/ })
/************************************************************************/
/******/ ({

/***/ 0:
/***/ (function(module, exports, __webpack_require__) {

module.exports = __webpack_require__("a1ec");


/***/ }),

/***/ "6b0d":
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
// runtime helper for setting properties on components
// in a tree-shakable way
exports.default = (sfc, props) => {
    const target = sfc.__vccOpts || sfc;
    for (const [key, val] of props) {
        target[key] = val;
    }
    return target;
};


/***/ }),

/***/ "8bbf":
/***/ (function(module, exports) {

module.exports = require("vue");

/***/ }),

/***/ "a1ec":
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
// ESM COMPAT FLAG
__webpack_require__.r(__webpack_exports__);

// EXTERNAL MODULE: external "vue"
var external_vue_ = __webpack_require__("8bbf");

// EXTERNAL MODULE: external "vue/server-renderer"
var server_renderer_ = __webpack_require__("f488");

// CONCATENATED MODULE: ./node_modules/cache-loader/dist/cjs.js??ref--13-0!./node_modules/thread-loader/dist/cjs.js!./node_modules/babel-loader/lib!./node_modules/vue-loader-v16/dist/templateLoader.js??ref--6!./node_modules/cache-loader/dist/cjs.js??ref--1-0!./node_modules/vue-loader-v16/dist??ref--1-1!./demo/App.vue?vue&type=template&id=09529b2f


function ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  const _component_v_surface = Object(external_vue_["resolveComponent"])("v-surface");
  const _component_v_layer = Object(external_vue_["resolveComponent"])("v-layer");
  const _component_v_circle = Object(external_vue_["resolveComponent"])("v-circle");
  const _component_HelloWorld = Object(external_vue_["resolveComponent"])("HelloWorld");
  _push(`<!--[--><h1>Demo</h1><p>This is a demo</p>`);
  _push(Object(server_renderer_["ssrRenderComponent"])(_component_v_surface, {
    width: "400",
    height: "500"
  }, {
    default: Object(external_vue_["withCtx"])((_, _push, _parent, _scopeId) => {
      if (_push) {
        _push(Object(server_renderer_["ssrRenderComponent"])(_component_v_layer, null, {
          default: Object(external_vue_["withCtx"])((_, _push, _parent, _scopeId) => {
            if (_push) {
              _push(Object(server_renderer_["ssrRenderComponent"])(_component_v_circle, {
                x: "80",
                y: "180",
                r: "150",
                fill: "ffffff",
                stroke: "ff00ff"
              }, null, _parent, _scopeId));
              _push(Object(server_renderer_["ssrRenderComponent"])(_component_HelloWorld, null, null, _parent, _scopeId));
            } else {
              return [Object(external_vue_["createVNode"])(_component_v_circle, {
                x: "80",
                y: "180",
                r: "150",
                fill: "ffffff",
                stroke: "ff00ff"
              }), Object(external_vue_["createVNode"])(_component_HelloWorld)];
            }
          }),
          _: 1
        }, _parent, _scopeId));
      } else {
        return [Object(external_vue_["createVNode"])(_component_v_layer, null, {
          default: Object(external_vue_["withCtx"])(() => [Object(external_vue_["createVNode"])(_component_v_circle, {
            x: "80",
            y: "180",
            r: "150",
            fill: "ffffff",
            stroke: "ff00ff"
          }), Object(external_vue_["createVNode"])(_component_HelloWorld)]),
          _: 1
        })];
      }
    }),
    _: 1
  }, _parent));
  _push(`<!--]-->`);
}
// CONCATENATED MODULE: ./demo/App.vue?vue&type=template&id=09529b2f

// CONCATENATED MODULE: ./node_modules/cache-loader/dist/cjs.js??ref--13-0!./node_modules/thread-loader/dist/cjs.js!./node_modules/babel-loader/lib!./node_modules/vue-loader-v16/dist/templateLoader.js??ref--6!./node_modules/cache-loader/dist/cjs.js??ref--1-0!./node_modules/vue-loader-v16/dist??ref--1-1!./demo/components/HelloWorld.vue?vue&type=template&id=06e14dfc


function HelloWorldvue_type_template_id_06e14dfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  const _component_v_layer = Object(external_vue_["resolveComponent"])("v-layer");
  const _component_v_rect = Object(external_vue_["resolveComponent"])("v-rect");
  const _component_v_round_rect = Object(external_vue_["resolveComponent"])("v-round-rect");
  const _component_v_triangle = Object(external_vue_["resolveComponent"])("v-triangle");
  _push(Object(server_renderer_["ssrRenderComponent"])(_component_v_layer, _attrs, {
    default: Object(external_vue_["withCtx"])((_, _push, _parent, _scopeId) => {
      if (_push) {
        _push(Object(server_renderer_["ssrRenderComponent"])(_component_v_rect, {
          x: "40",
          y: "40",
          width: "100",
          height: "100",
          fill: "000000",
          stroke: "ffffff"
        }, {
          default: Object(external_vue_["withCtx"])((_, _push, _parent, _scopeId) => {
            if (_push) {
              _push(`<!--[-->`);
              Object(server_renderer_["ssrRenderList"])($data.list, item => {
                _push(Object(server_renderer_["ssrRenderComponent"])(_component_v_rect, {
                  x: 20 + item * 50,
                  y: 20 + item * 40,
                  width: "50",
                  height: "50",
                  fill: "00ffaa",
                  stroke: "000000"
                }, null, _parent, _scopeId));
              });
              _push(`<!--]-->`);
            } else {
              return [(Object(external_vue_["openBlock"])(true), Object(external_vue_["createBlock"])(external_vue_["Fragment"], null, Object(external_vue_["renderList"])($data.list, item => {
                return Object(external_vue_["openBlock"])(), Object(external_vue_["createBlock"])(_component_v_rect, {
                  x: 20 + item * 50,
                  y: 20 + item * 40,
                  width: "50",
                  height: "50",
                  fill: "00ffaa",
                  stroke: "000000"
                }, null, 8, ["x", "y"]);
              }), 256))];
            }
          }),
          _: 1
        }, _parent, _scopeId));
        _push(Object(server_renderer_["ssrRenderComponent"])(_component_v_round_rect, {
          x: "220",
          y: "50",
          width: "80",
          height: "80",
          radius: "10",
          fill: "ee22ee",
          stroke: "ffffff"
        }, null, _parent, _scopeId));
        _push(Object(server_renderer_["ssrRenderComponent"])(_component_v_triangle, {
          x: "20",
          y: "220",
          width: "150",
          height: "100",
          fill: "ffffff",
          stroke: "00aaff"
        }, null, _parent, _scopeId));
        _push(Object(server_renderer_["ssrRenderComponent"])(_component_v_triangle, {
          x: "100",
          y: "260",
          width: "120",
          height: "120",
          fill: "00aaff"
        }, null, _parent, _scopeId));
      } else {
        return [Object(external_vue_["createVNode"])(_component_v_rect, {
          x: "40",
          y: "40",
          width: "100",
          height: "100",
          fill: "000000",
          stroke: "ffffff"
        }, {
          default: Object(external_vue_["withCtx"])(() => [(Object(external_vue_["openBlock"])(true), Object(external_vue_["createBlock"])(external_vue_["Fragment"], null, Object(external_vue_["renderList"])($data.list, item => {
            return Object(external_vue_["openBlock"])(), Object(external_vue_["createBlock"])(_component_v_rect, {
              x: 20 + item * 50,
              y: 20 + item * 40,
              width: "50",
              height: "50",
              fill: "00ffaa",
              stroke: "000000"
            }, null, 8, ["x", "y"]);
          }), 256))]),
          _: 1
        }), Object(external_vue_["createVNode"])(_component_v_round_rect, {
          x: "220",
          y: "50",
          width: "80",
          height: "80",
          radius: "10",
          fill: "ee22ee",
          stroke: "ffffff"
        }), Object(external_vue_["createVNode"])(_component_v_triangle, {
          x: "20",
          y: "220",
          width: "150",
          height: "100",
          fill: "ffffff",
          stroke: "00aaff"
        }), Object(external_vue_["createVNode"])(_component_v_triangle, {
          x: "100",
          y: "260",
          width: "120",
          height: "120",
          fill: "00aaff"
        })];
      }
    }),
    _: 1
  }, _parent));
}
// CONCATENATED MODULE: ./demo/components/HelloWorld.vue?vue&type=template&id=06e14dfc

// CONCATENATED MODULE: ./node_modules/cache-loader/dist/cjs.js??ref--13-0!./node_modules/thread-loader/dist/cjs.js!./node_modules/babel-loader/lib!./node_modules/cache-loader/dist/cjs.js??ref--1-0!./node_modules/vue-loader-v16/dist??ref--1-1!./demo/components/HelloWorld.vue?vue&type=script&lang=js
/* harmony default export */ var HelloWorldvue_type_script_lang_js = ({
  name: 'HelloWorld',
  data() {
    return {
      list: [0, 1, 2]
    };
  }
});
// CONCATENATED MODULE: ./demo/components/HelloWorld.vue?vue&type=script&lang=js
 
// EXTERNAL MODULE: ./node_modules/vue-loader-v16/dist/exportHelper.js
var exportHelper = __webpack_require__("6b0d");
var exportHelper_default = /*#__PURE__*/__webpack_require__.n(exportHelper);

// CONCATENATED MODULE: ./demo/components/HelloWorld.vue





const __exports__ = /*#__PURE__*/exportHelper_default()(HelloWorldvue_type_script_lang_js, [['ssrRender',HelloWorldvue_type_template_id_06e14dfc_ssrRender]])

/* harmony default export */ var HelloWorld = (__exports__);
// CONCATENATED MODULE: ./node_modules/cache-loader/dist/cjs.js??ref--13-0!./node_modules/thread-loader/dist/cjs.js!./node_modules/babel-loader/lib!./node_modules/cache-loader/dist/cjs.js??ref--1-0!./node_modules/vue-loader-v16/dist??ref--1-1!./demo/App.vue?vue&type=script&lang=js

/* harmony default export */ var Appvue_type_script_lang_js = ({
  name: 'App',
  components: {
    HelloWorld: HelloWorld
  }
});
// CONCATENATED MODULE: ./demo/App.vue?vue&type=script&lang=js
 
// CONCATENATED MODULE: ./demo/App.vue





const App_exports_ = /*#__PURE__*/exportHelper_default()(Appvue_type_script_lang_js, [['ssrRender',ssrRender]])

/* harmony default export */ var App = (App_exports_);
// CONCATENATED MODULE: ./demo/main.js

/* harmony default export */ var main = __webpack_exports__["default"] = (App);

/***/ }),

/***/ "f488":
/***/ (function(module, exports) {

module.exports = require("vue/server-renderer");

/***/ })

/******/ });