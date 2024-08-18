import { createApp } from "vue";
import App from "./App.vue";
import { VueSkia } from 'vue-skia'
import Vue3Progress from "vue3-progress";

const app = createApp(App);
app.use(VueSkia);
app.use(Vue3Progress, {
    position: "fixed",
    color: "rgb(0, 161, 132)",
  })
app.mount("#app");
