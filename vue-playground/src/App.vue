<template>
  <HelloWorld msg="Vue Skia " />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import HelloWorld from "./components/HelloWorld.vue";

export default defineComponent({
  name: "App",
  components: {
    HelloWorld,
  },
  data() {
    return {
      HelloWorld
    }
  }
});
</script>

<style>
body {
  margin: 20px;
  padding: 0;
}

textarea {
  outline: none;
}

</style>
