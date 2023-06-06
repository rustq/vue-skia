<template>
    <v-sk-points :points="points" :strokeWidth="strokeWidth" :style="style" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "VPoints",
    props: {
        points: Array,
        strokeWidth: Number,
        style: String,
        color: String,
    },
});
</script>
    