<template>
    <v-sk-circle :cx="cx" :cy="cy" :r="r" :style="style" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "VCircle",
    props: {
        cx: Number,
        cy: Number,
        r: Number,
        style: String,
        color: String,
    },
});
</script>
    