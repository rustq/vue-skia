# Vue Skia

[![license](https://img.shields.io/badge/license-MIT-cyan)](https://revolunet.mit-license.org/) ![vue-3.x](https://img.shields.io/badge/vue-3.x-lightgreen) [![npm](https://img.shields.io/npm/v/vue-skia?vue-skia)](https://www.npmjs.com/package/vue-skia) [![crates](https://img.shields.io/crates/v/soft-skia)](https://crates.io/crates/soft_skia)


The `vue-skia` is a `skia` based 2d graphics `vue` rendering library. It is based on `Rust` to implement software rasterization to perform rendering. It takes up less memory than the native canvas, however it is still a experiment project. And it's based entirely on `vue` syntax.

基于 `Skia` 的 2D 图形 `Vue` 渲染库 —— 使用 `Rust` 语言实现纯软件光栅化渲染，相比原生画布占用更少的内存，不过目前仍然是一个实验项目；此外使用层面完全基于 `Vue` 语法。


[![usage](https://user-images.githubusercontent.com/11075892/260789003-8bc6cf06-1525-468a-ad70-357771e9969f.png)](https://vue-skia.netlify.app)

Live Demo: [https://vue-skia.netlify.app](https://vue-skia.netlify.app)

## Usage

```shell
$ npm i vue-skia
```

`main.ts`

```ts
import { createApp } from "vue";
import App from "./App.vue";
import { VueSkia } from 'vue-skia'

const app = createApp(App);
app.use(VueSkia);
app.mount("#app");
```

`App.vue`

```vue
<template v-if="!loading">
  <v-surface :width="360" :height="360">
    <v-rect :x="10" :y="220" :width="30" :height="30" color="cyan" :style="'fill'" />
    <v-line :strokeWidth="8" color="black" :p1="[100, 260]" :p2="[50, 285]" />
    <v-round-rect :x="220" :y="50" :width="80" :height="80" :r="10" color="fuchsia" :style="'stroke'" />
    <v-circle :cx="200" :cy="260" :r="50" :style="'stroke'" color="fuchsia" />
    <v-points :points="[
        [138, 10],
        [178, 90],
        [266, 103],
        [202, 165],
        [217, 254],
        [138, 212],
        [59, 254],
        [74, 165],
        [10, 103],
        [98, 90],
        [138, 10],
      ]" :style="'fill'" :strokeWidth="1" :color="'rgba(200, 255, 0, 0.7)'" />
  </v-surface>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import launch, { VSurface, VRect, VCircle, VRoundRect, VLine, VPoints } from "vue-skia";

export default defineComponent({
  name: "HelloWorld",
  components: {
    VSurface,
    VRect,
    VCircle,
    VRoundRect,
    VLine,
    VPoints
  },
  data() {
    return {
      loading: true,
    };
  },
  mounted() {
    launch().then(() => {
      this.loading = false;
    });
  },
});
</script>
```

## Library Development

#### Soft Skia Development

```shell
$ cd soft-skia
$ cargo test
```

#### Soft Skia WASM Development

```shell
$ cd soft-skia-wasm
$ wasm-pack build --release --target web
```

#### Vue Skia Framework Development

```shell
$ cd vue-skia-framework
$ npm i
$ npm run build
```

## License

[MIT](https://opensource.org/licenses/MIT)
