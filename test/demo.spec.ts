import { render } from "../";
import { App as VueApp } from "vue";

const path = require("path");
const manifest = require("../dist/ssr-manifest.json");
const appPath = path.join(__dirname, "../dist", manifest["app.js"]);
const App = require(appPath).default as VueApp;

render(App).then(({ encodePNG }) => {
    encodePNG("./demo.spec.png")
})