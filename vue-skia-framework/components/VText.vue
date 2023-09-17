<template>
  <v-sk-text :text="text" :x="x" :y="y" :fontSize="fontSize" />
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
  name: "VText",
  props: {
    text: {
      type: String as PropType<string>,
      required: true,
    },
    x: {
      type: Number as PropType<number>,
      required: true,
    },
    y: {
      type: Number as PropType<number>,
      required: true,
    },
    fontSize: {
      type: Number as PropType<number>,
      required: true,
    },
  },
});
</script>
