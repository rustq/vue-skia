<template>
    <v-sk-circle :cx="cx" :cy="cy" :r="r" :style="style" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
    name: "VCircle",
    props: {
        cx: {
            type: Number as PropType<number>,
            required: true
        },
        cy: {
            type: Number as PropType<number>,
            required: true
        },
        r: {
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
    