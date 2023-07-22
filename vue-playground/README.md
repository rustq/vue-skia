# vue-playground

## Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run serve
```

### How to debug the vue syntax code in playground

`App.vue`

```diff
  data() {
    return {
      CustomLayout: markRaw(CustomLayout),
      loading: true,
      count: 2,
      VSurface,
      VRect,
      VCircle,
      VRoundRect,
      VLine,
      VPoints,
      code,
      LoadingCode,
-      debug: false,
+      debug: true,
      error: undefined,
    };
  },
```

### How to debug the wasm in vue-playgrounnd

```shell
$ rm -rf {YOUR_PROJECT_WORK_SPACE}/vue-playground/node_modules/vue-skia/soft-skia-wasm/pkg
$ ln -s {YOUR_PROJECT_WORK_SPACE}/soft-skia-wasm/pkg {YOUR_PROJECT_WORK_SPACE}/vue-playground/node_modules/vue-skia/soft-skia-wasm/pkg
```
