<template>
    <v-sk-surface :width="width" :height="height">
        <slot></slot>
    </v-sk-surface>
</template>
    
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "VSurface",
    props: {
        width: Number,
        height: Number,
    },
});
</script>
    