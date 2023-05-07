# Vue Skia

[![license](https://img.shields.io/npm/l/vue-skia?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/vue-skia?vue-skia?color=lightgreen)](https://www.npmjs.com/package/vue-skia)


`Skia-based 2D graphic software rasterization Vue rendering framework`

`基于 Skia 的 2D 图形软件光栅化 Vue 渲染库`


## Demo

https://github.com/rustq/vue-skia/blob/2edb1553e47211fb750dc42b63751cec34dde891/vue-playground/src/components/HelloWorld.vue#L8-L21


## Development

```shell
$ git clone git@github.com:rustq/vue-skia.git

$ cd vue-skia
```

```shell
$ cd soft-skia
$ cargo test
$ cargo build
```

```shell
$ cd ../soft-skia-wasm
$ cargo test
$ wasm-pack build --release --target web
```

```shell
$ cd ../vue-playground
$ yarn serve
```


## v0.1.0 TODO

- [x] Soft Skia
- [ ] Soft Skia WASM
- [ ] Vue Skia Framework
- [ ] Others All


## License

[MIT](https://opensource.org/licenses/MIT)
