<template>
  <template v-if="!loading">
    <VueLive :code="code" :components="{
        VSurface,
        VRect,
        VCircle,
        VRoundRect,
        VLine,
        VPoints,
      }" @error="(e) => { e }" />
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
      global: window
    };
  },
  mounted() {
    launch().then(() => {
      this.loading = false;
    });
  },
});
</script>

