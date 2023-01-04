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
  const ctx = core.Canvas_create_a_context();
  console.log(core.Context_read_cc(ctx));
  core.Context_make_a_triangle(ctx, 0, 0);
  core.Context_make_a_triangle(ctx, 100, 100);
  core.Context_make_a_triangle(ctx, 140, 20);
  core.Context_make_a_circle(ctx);
  core.Context_make_a_draw(ctx);
})