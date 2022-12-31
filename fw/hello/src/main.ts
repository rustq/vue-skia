import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import FW from "../../index";

import "./assets/main.css";

const app = createApp(App);

app.use(router);
app.use(FW);
app.mount("#app");
