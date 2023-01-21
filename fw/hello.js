const { createSSRApp } = require('vue');
const { renderToString } = require('vue/server-renderer');
// const FW = require('./fw.js').default;
const { parse } = require('node-html-parser');
const core = require('../core/lib/index.node');
const App = require('./dist/app.vue').default;

const app = createSSRApp(App)

// app.use(FW);

renderToString(app).then((html) => {
  console.log(html)
  const ctx = core.createContext();
  const root = parse(html);
  core.createRoundRect(ctx, 0, 0, 80, 80, 20, "0000ff", "ffffff");
  core.createRect(ctx, 56, 14, 20, 20, "ffffff", "ff22aa");
  core.createRect(ctx, 156, 114, 20, 20, "ffffff", "ff22aa");
  function recursiveTraceChild(root) {
    root.childNodes.forEach(child => {
      console.log(child.rawTagName)
      if (child.rawTagName === 'v-circle') {
        const x = Number(child.getAttribute("x"));
        const y = Number(child.getAttribute("y"));
        core.createCircle(ctx, x, y, 30, "003333", "000011");
      }
      if (child.rawTagName === 'v-rect') {
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