import { render } from "../";
import { App as VueApp } from "vue";

const path = require("path");
const manifest = require("../demo/dist/ssr-manifest.json");
const appPath = path.join(__dirname, "../demo/dist", manifest["app.js"]);
const App = require(appPath).default as VueApp;

render(App).then(({ encodePNG }) => {
    encodePNG("./demo.spec.png")
})