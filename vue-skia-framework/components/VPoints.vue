<template>
    <v-sk-points :points="points" :strokeWidth="strokeWidth" :style="style" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
    name: "VPoints",
    props: {
        points: {
            type: Array as PropType<[number, number][]>,
            required: true
        },
        strokeWidth: {
            type: Number as PropType<number>,
            required: false
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
    