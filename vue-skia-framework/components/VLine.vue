<template>
    <v-sk-line :p1="p1" :p2="p2" :strokeWidth="strokeWidth" :color="color" />
</template>
    
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "VLine",
    props: {
        p1: Array,
        p2: Array,
        strokeWidth: Number,
        color: String,
    },
});
</script>
    