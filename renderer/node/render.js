const path = require("path");
const express = require("express");
const { createSSRApp } = require("vue");
const { renderToString } = require("@vue/server-renderer");
const manifest = require("./dist/ssr-manifest.json");
const vsk = require('./vsk').default;
const { parse } = require('node-html-parser');
const core = require('../../core/lib/index.node');

const server = express();

const appPath = path.join(__dirname, "./dist", manifest["app.js"]);
const App = require(appPath).default;

const app = createSSRApp(App);

app.use(vsk);

renderToString(app).then(appContent => {
  console.log('[appContent]', appContent);

  const ctx = core.createContext();
  const root = parse(appContent);
  function recursiveTraceChild(root) {
    root.childNodes.forEach(child => {
      console.log(child.rawTagName)
      if (child.rawTagName === 'Circle') {
        const x = Number(child.getAttribute("x"));
        const y = Number(child.getAttribute("y"));
        core.createCircle(ctx, x, y, 30, "003333", "000011");
      }
      if (child.rawTagName === 'Rect') {
        const x = Number(child.getAttribute("x"));
        const y = Number(child.getAttribute("y"));
        core.createTriangle(ctx, x, y, x + Math.random() * 60, y + Math.random() * 20, x - Math.random() * 20, y - Math.random() * 20, "ff3eff", "ffffff");
      }
      if (child.rawTagName === 'Layer') {
        const x = Number(child.getAttribute("x"));
        const y = Number(child.getAttribute("y"));
        core.createRect(ctx, 0, 0, 120, 120, "cccccc", "ffffff");
      }
      return recursiveTraceChild(child);
    })
  }
  recursiveTraceChild(root)

  core.createRoundRect(ctx, 0, 0, 80, 80, 20, "0000ff", "ffffff");
  core.createRect(ctx, 56, 14, 20, 20, "ffffff", "ff22aa");
  core.createRect(ctx, 156, 114, 20, 20, "ffffff", "ff22aa");

  core.save(ctx, "hello.png");
  console.log('done 🌟')
});

