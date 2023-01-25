# Vue Skia

[![license](https://img.shields.io/npm/l/vue-skia?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/vue-skia?vue-skia?color=lightgreen)](https://www.npmjs.com/package/vue-skia)


`High-performance 2d Graphics for Vue using Skia`

`基于 Skia 的高性能 2D 图形渲染的 Vue 绘制库`


![slogan](https://user-images.githubusercontent.com/11075892/214504036-8e28819e-0cc8-4177-a681-8a2d1680274f.png)



## Install

```shell
# It depends on cargo-cp-artifact for postinstall compile, so you need to install cargo first.
# 因为在安装后的编译阶段需要 cargo-cp-artifact 需要确保已经安装了 cargo

$ npm i vue-skia
```




## Usage

```vue
<template>
  <v-surface width="400" height="500">
    <v-circle x="80" y="180" r="150" fill="ffffff" stroke="ff00ff"/>
  </v-surface>
</template>
```

```typescript
import { render } from "vue-skia";

render(App).then(({ encodePNG }) => {
    encodePNG("./demo.spec.png")
})
```
## Demo

https://github.com/rustq/vue-skia/blob/477825ba3ba4f3102db183eb260f060c0b9193fa/demo/App.vue#L1-L21

https://github.com/rustq/vue-skia/blob/477825ba3ba4f3102db183eb260f060c0b9193fa/test/demo.spec.ts#L1-L11


## APIs

#### surface

```vue
<v-surface width="400" height="500">
</v-surface>
```

#### rect

```vue
<v-rect x="40" y="40" width="100" height="100" fill="000000" stroke="ffffff" />
```

#### round rect

```vue
<v-round-rect x="220" y="50" width="80" height="80" radius="10" fill="ee22ee" stroke="ffffff" />
```

#### circle

```vue
<v-circle x="80" y="180" r="150" fill="ffffff" stroke="ff00ff" />
```


## Development

```shell
$ git clone git@github.com:rustq/vue-skia.git

$ cd vue-skia

$ npm i
```

```
$ npm run build
```

```shell
$ npm run build:demo

$ npm run test
```


## v0.1.0 TODO

- [x] in Node.js

- [ ] in Mobile

- [ ] in Web


## License

[MIT](https://opensource.org/licenses/MIT)
