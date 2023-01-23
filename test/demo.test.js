const path = require("path");
const manifest = require("../dist/ssr-manifest.json");

const appPath = path.join(__dirname, "../dist", manifest["app.js"]);
const App = require(appPath).default;

const v = require("../").default;

v(App, path.join(__dirname, "demo.test.png"))