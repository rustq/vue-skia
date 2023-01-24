import { App as VueApp, createSSRApp } from "vue";
import vsk from "../../plugin";

const { renderToString } = require("@vue/server-renderer");
const { parse } = require('node-html-parser');
const core = require('../../core');

function getSurface(root) {
  for (let i = 0; i < root.childNodes.length; i++) {
    const child = root.childNodes[i];
    if (!child?.rawTagName) {
      continue;
    }
    if (child.rawTagName === 'Surface') {
      return child;
    } else {
      getSurface(child);
    }
  }
}

function recursivePaintChild(root, ctx) {
  for (let i = 0; i < root.childNodes.length; i++) {
    const child = root.childNodes[i];
    if (!child?.rawTagName) {
      continue;
    }
    const x = Number(child.getAttribute("x"));
    const y = Number(child.getAttribute("y"));
    const fill = child.getAttribute("fill") || "000000";
    const stroke = child.getAttribute("stroke");
    switch (child.rawTagName) {
      case 'Circle': {
        const r = Number(child.getAttribute("r"));
        core.createCircle(ctx, x, y, r, fill, stroke);
        break;
      }
      case 'Rect': {
        const width = Number(child.getAttribute("width"));
        const height = Number(child.getAttribute("height"));
        core.createRect(ctx, x, y, width, height, fill, stroke);
        break;
      }
      case 'RoundRect': {
        const width = Number(child.getAttribute("width"));
        const height = Number(child.getAttribute("height"));
        const radius = Number(child.getAttribute("radius"));
        core.createRoundRect(ctx, x, y, width, height, radius, fill, stroke);
        break;
      }
      case 'Triangle': {
        const width = Number(child.getAttribute("width"));
        const height = Number(child.getAttribute("height"));
        core.createTriangle(ctx, x, y + height, x + width, y + height, x + width / 2, y, fill, stroke);
        break;
      }
      default: {
      }
    }
    recursivePaintChild(child, ctx);
  }
}

export function render(App: VueApp): Promise<{ encodePNG: (path: string) => void }> {

  return new Promise(resolve => {
    const app = createSSRApp(App);
    app.use(vsk);
    renderToString(app).then((appContent: string) => {
      const root = parse(appContent);
      const surface = getSurface(root);
      if (!surface) {
        console.error('No surface');
        return;
      }
      const width = Number(surface.getAttribute("width"));
      const height = Number(surface.getAttribute("height"));
      const ctx = core.createContext(width, height);
      recursivePaintChild(surface, ctx);
      resolve({
        encodePNG: (path: string) => {
          core.encodePNG(ctx, path);
        }
      })
    });
  })
  
}
