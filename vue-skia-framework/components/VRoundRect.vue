<template>
    <v-sk-roundRect :x="x" :y="y" :r="r" :width="width" :height="height" :style="style" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "VRoundRect",
    props: {
        x: Number,
        y: Number,
        r: Number,
        width: Number,
        height: Number,
        style: String,
        color: String,
    },
});
</script>
    