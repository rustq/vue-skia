<template>
  <v-sk-image :image="base64String" :x="x" :y="y" :width="width" :height="height" />
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
  name: "VImage",
  props: {
    image: {
      type: String as PropType<string>,
      required: true
    },
    x: {
      type: Number as PropType<number>,
      required: true,
    },
    y: {
      type: Number as PropType<number>,
      required: true,
    },
    width: {
      type: Number as PropType<number>,
      required: true,
    },
    height: {
      type: Number as PropType<number>,
      required: true,
    }
  },
  setup(props){
    console.log(props.image);
    const base64String = props.image.replace("data:image/png;base64,", "")
    return {
      base64String
    }
  }
});
</script>
