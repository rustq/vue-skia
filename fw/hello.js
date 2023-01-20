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
  const ctx = core.createContext();
  const root = parse(html);
  core.createRect(ctx, 0, 0, 20, 20, "ffffff");
  function recursiveTraceChild(root) {
    root.childNodes.forEach(child => {
      if (child.rawTagName === 'Circle') {
        const x = Number(child.getAttribute("x"));
        const y = Number(child.getAttribute("y"));
        core.createCircle(ctx, x, y, 30, "003333");
      }
      if (child.rawTagName === 'Rect') {
        const x = Number(child.getAttribute("x"));
        const y = Number(child.getAttribute("y"));
        core.createTriangle(ctx, x, y, x + Math.random() * 60, y + Math.random() * 20, x - Math.random() * 20, y - Math.random() * 20, "ff3eff", "ffffff");
      }
      return recursiveTraceChild(child);
    })
  }
  recursiveTraceChild(root)
  core.save(ctx, "hello.png");
  console.log('done 🌟')
})