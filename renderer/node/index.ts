const { renderToString } = require("@vue/server-renderer");
const { createSSRApp } = require("vue");
const { parse } = require('node-html-parser');
const vsk = require('../../plugin/').default;
const core = require('../../core');

function vrender(App: any, path: string) {

  const app = createSSRApp(App);
  app.use(vsk);
  
  renderToString(app).then((appContent: string) => {
    console.log('[appContent]', appContent);
  
    const ctx = core.createContext();
    const root = parse(appContent);
    function recursiveTraceChild(root: any) {
      root.childNodes.forEach((child: any) => {
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
          core.createRect(ctx, x, y, 120, 120, "cccccc", "ffffff");
        }
        return recursiveTraceChild(child);
      })
    }
    recursiveTraceChild(root as any)
  
    core.createRoundRect(ctx, 0, 0, 80, 80, 20, "0000ff", "ffffff");
    core.createRect(ctx, 56, 14, 20, 20, "ffffff", "ff22aa");
    core.createRect(ctx, 156, 114, 20, 20, "ffffff", "ff22aa");
  
    core.save(ctx, path);
    console.log('vrender done pl 🌟')
  });
  
  
}

module.exports.default = vrender