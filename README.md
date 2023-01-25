# Vue Skia

[![license](https://img.shields.io/npm/l/vue-skia?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/vue-skia?vue-skia?color=lightgreen)](https://www.npmjs.com/package/vue-skia)


`High-performance 2d Graphics for Vue using Skia`

`基于 Skia 的高性能 2D 图形渲染的 Vue 绘制库`


![slogan](https://user-images.githubusercontent.com/11075892/214504036-8e28819e-0cc8-4177-a681-8a2d1680274f.png)


## v0.1.0 TODO

- [x] in Node.js

- [ ] in Mobile

- [ ] in Web

## Usage

```shell
$ npm i vue-skia
```

It depends on `cargo-cp-artifact` for postinstall compile, so you need to install `cargo` first.

因为在安装后的编译阶段需要 `cargo-cp-artifact` 需要确保已经安装了 `cargo`


## Demo

```vue
<template>
  <v-surface width="400" height="500">
    <v-circle x="80" y="180" r="150" fill="ffffff" stroke="ff00ff"/>
  </v-surface>
</template>
```

Demo Code: [App.vue](./demo/App.vue)

#### Build Demo

```shell
$ npm run build:demo
```

#### Run Test

```shell
$ npm run test
```

## Building

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

## License

[MIT](https://opensource.org/licenses/MIT)
