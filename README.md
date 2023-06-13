![slogan](https://user-images.githubusercontent.com/11075892/214504036-8e28819e-0cc8-4177-a681-8a2d1680274f.png)

# Vue Skia

[![license](https://img.shields.io/npm/l/vue-skia?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/vue-skia?vue-skia?color=lightgreen)](https://www.npmjs.com/package/vue-skia)


`Skia-based 2D graphic software rasterization Vue rendering framework`

`基于 Skia 的 2D 图形软件光栅化 Vue 渲染库`


## Usage

```shell
$ npm i vue-skia
```

https://github.com/rustq/vue-skia/blob/2587a6216d98b7d3d093781ec3e8c5740bc7df84/vue-playground/src/main.ts#L1-L7

https://github.com/rustq/vue-skia/blob/2587a6216d98b7d3d093781ec3e8c5740bc7df84/vue-playground/src/components/HelloWorld.vue#L1-L117

![usage](https://user-images.githubusercontent.com/11075892/245521765-e5c8093d-bdd3-41e4-9f10-d3a6650dd55f.png)

## Demo Getting Started

```shell
$ git clone git@github.com:rustq/vue-skia.git
```

```shell
$ cd vue-playground
$ npm i
$ npm run serve
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
