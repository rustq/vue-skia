<template>
    <v-sk-line :p1="p1" :p2="p2" :strokeWidth="strokeWidth" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
    name: "VLine",
    props: {
        p1: {
            type: Array as PropType<number[]>,
            required: true
        },
        p2: {
            type: Array as PropType<number[]>,
            required: true
        },
        strokeWidth: {
            type: Number as PropType<number>,
            required: true
        },
        color: {
            type: String as PropType<string>,
            required: true
        },
    },
});
</script>
    