<template>
    <v-sk-roundRect :x="x" :y="y" :r="r" :width="width" :height="height" :style="style" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
    name: "VRoundRect",
    props: {
        x: {
            type: Number as PropType<number>,
            required: true
        },
        y: {
            type: Number as PropType<number>,
            required: true
        },
        r: {
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
            required: false
        },
        color: {
            type: String as PropType<string>,
            required: false
        },
    },
});
</script>
    