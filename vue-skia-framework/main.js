import launch from "./lib/launch";
import plugin from "./lib/plugin";
import VSurface from "./components/VSurface.vue";
import VGroup from "./components/VGroup.vue";
import VRect from "./components/VRect.vue";
import VCircle from "./components/VCircle.vue";
import VRoundRect from "./components/VRoundRect.vue";
import VLine from "./components/VLine.vue";
import VPoints from "./components/VPoints.vue";
import VImage from './components/VImage.vue';

export default launch;
export { plugin as VueSkia }
export { VSurface, VGroup, VRect, VCircle, VRoundRect, VLine, VPoints, VImage }