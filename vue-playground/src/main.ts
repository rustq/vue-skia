import { createApp } from "vue";
import App from "./App.vue";
import VueSkia from '../../vue-skia-framework/plugin/index'

const app = createApp(App);
app.use(VueSkia);
app.mount("#app");
