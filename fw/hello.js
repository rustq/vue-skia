const { createSSRApp } = require('vue');
const { renderToString } = require('vue/server-renderer');
const FW = require('./fw.js').default;
const { parse } = require('node-html-parser');
const core = require('../core/lib/index.node');

const app = createSSRApp({
    data: () => ({ count: 1 }),
    template: `
      <div>
        <button @click="count++">{{ count }}</button>
        <div>
          <v-layer>
            <v-rect x="100" :y="count * 300">
              <v-circle :size="count * 8" :x="19 - count" y="30" />
              <v-circle :x="20" y="130" />
            </v-rect>
            <v-rect x="120" :y="count * 200" />
            <v-rect x="20" :y="count * 100" />
            <v-rect x="120" :y="count * 80" />
            </v-layer>
        </div>
      </div>`,
  })

app.use(FW);

renderToString(app).then((html) => {
  console.log(html)
  const ctx = core.Canvas_create_a_context();
  const root = parse(html);
  function recursiveTraceChild(root) {
    root.childNodes.forEach(child => {
      if (child.rawTagName === 'Circle') {
        const x = Number(child.getAttribute("x"));
        const y = Number(child.getAttribute("y"));
        core.Context_make_a_circle(ctx, x, y);
      }
      if (child.rawTagName === 'Rect') {
        const x = Number(child.getAttribute("x"));
        const y = Number(child.getAttribute("y"));
        core.Context_make_a_triangle(ctx, x, y);
      }
      return recursiveTraceChild(child);
    })
  }
  recursiveTraceChild(root)
  core.Context_make_a_draw(ctx);
  console.log('done 🌟')
})