# Vue Skia

[![license](https://img.shields.io/npm/l/vue-skia?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/vue-skia?vue-skia?color=lightgreen)](https://www.npmjs.com/package/vue-skia)


`The vue-skia is a skia-based 2d graphics vue rendering library. It is based on Rust to implement software rasterization to perform rendering. It takes up less memory than the native canvas, however it is still a experiment project. And it's based entirely on vue syntax.`

`基于 Skia 的 2D 图形 Vue 渲染库 —— 它使用 Rust 语言实现软件光栅化来执行渲染，相比原生画布占用更少的内存；不过仍然是一个实验项目，在使用层面完全基于 Vue 语法。`


![usage](https://user-images.githubusercontent.com/11075892/245521765-e5c8093d-bdd3-41e4-9f10-d3a6650dd55f.png)

## Demo

[Live Demo](https://vue-skia.netlify.app/)

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
<template>
  <v-surface :width="400" :height="400">
    <v-points :points="[
      [128, 0],
      [168, 80],
      [256, 93],
      [192, 155],
      [207, 244],
      [128, 202],
      [49, 244],
      [64, 155],
      [0, 93],
      [88, 80],
      [128, 0],
    ]" :style="'fill'" :strokeWidth="1" :color="'rgba(200, 255, 0, 0.7)'"></v-points>
    <v-circle :cx="200" :cy="260" :r="80" :style="'stroke'" color="#ee22ee" />
    <v-rect :x="10" :y="220" :width="30" :height="30" color="#00aaff"
        :style="'fill'">
    </v-rect>
    <v-round-rect :x="220" :y="50" :width="80" :height="80" :r="10" color="#ee22ee" :style="'stroke'" />
    <v-points :style="'fill'" :strokeWidth="2" color="#00aaff" :points="[
        [100, 260],
        [80, 300],
        [120, 300],
        ]" />
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
