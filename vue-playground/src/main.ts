import { createApp } from "vue";
import App from "./App.vue";
import VueSkia from '../../plugin/index'

const app = createApp(App);
app.use(VueSkia);
app.mount("#app");
