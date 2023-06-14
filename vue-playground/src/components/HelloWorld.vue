<template>
  <template v-if="!loading && !debug">
    <VueLive :code="code" :components="{
        VSurface,
        VRect,
        VCircle,
        VRoundRect,
        VLine,
        VPoints,
      }" @error="(e) => { e }" />
  </template>
  <template v-if="!loading && debug">
    <v-surface :width="400" :height="400">
      <v-points :points="[
        [128, 0],
        [168, 80],
        [256, 93],
        [192, 155],
        [207, 244],
        [128, 202],
        [49, 244],
        [64, 155],
        [0, 93],
        [88, 80],
        [128, 0],
      ]" :style="'fill'" :strokeWidth="1" :color="'rgba(200, 255, 0, 1)'">
      </v-points>
      <v-circle :cx="200" :cy="260" :r="80" :style="'stroke'" color="#ee22ee" />
      <v-rect :x="10" :y="220" :width="30" :height="30" color="#00aaff"
        :style="'fill'">
      </v-rect>
    </v-surface>
  </template>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import launch, { VSurface, VRect, VCircle, VRoundRect, VLine, VPoints } from "vue-skia";
import { VueLive } from "vue-live";
import "vue-live/style.css";
import "prism-themes/themes/prism-ghcolors.css";
import code from "./code";

export default defineComponent({
  name: "HelloWorld",
  props: {
    msg: String,
  },
  components: {
    VSurface,
    VRect,
    VCircle,
    VRoundRect,
    VLine,
    VPoints,
    VueLive,
  },
  data() {
    return {
      loading: true,
      count: 2,
      VSurface,
      VRect,
      VCircle,
      VRoundRect,
      VLine,
      VPoints,
      code,
      debug: true,
    };
  },
  mounted() {
    launch().then(() => {
      this.loading = false;
    });
  },
});
</script>

