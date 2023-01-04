const { createSSRApp } = require('vue');
const { renderToString } = require('vue/server-renderer');
const FW = require('./fw.js');
const { parse } = require('node-html-parser');
const core = require('../core/lib/index.node');

const app = createSSRApp({
    data: () => ({ count: 1 }),
    template: `<div><button @click="count++">{{ count }}</button><div><v-layer><v-rect><v-circle :size="count * 8" :x="19 - count" /></v-rect></v-layer></div></div>`,
  })

app.use(FW);

renderToString(app).then((html) => {
  const root = parse(html);
  console.log(root.childNodes[0].rawTagName)
  console.log(core)
  core.Canvas_draw_circle();
})