<template>
    <v-sk-rect :x="x" :y="y" :width="width" :height="height" :style="style" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "VRect",
    props: {
        x: Number,
        y: Number,
        width: Number,
        height: Number,
        style: String,
        color: String,
    },
});
</script>
    