<template>
    <v-sk-rect :x="x" :y="y" :width="width" :height="height" :style="style" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
    name: "VRect",
    props: {
        x: {
            type: Number as PropType<number>,
            required: true
        },
        y: {
            type: Number as PropType<number>,
            required: true
        },
        width: {
            type: Number as PropType<number>,
            required: true
        },
        height: {
            type: Number as PropType<number>,
            required: true
        },
        style: {
            type: String as PropType<'fill' | 'stroke'>,
            required: true
        },
        color: {
            type: String as PropType<string>,
            required: true
        },
    },
});
</script>
    