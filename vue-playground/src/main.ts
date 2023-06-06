import { createApp } from "vue";
import App from "./App.vue";
import VueSkia from '../../vue-skia-framework/lib/plugin'

const app = createApp(App);
app.use(VueSkia);
app.mount("#app");
