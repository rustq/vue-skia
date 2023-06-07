<template>
    <v-sk-surface :width="width" :height="height">
        <slot></slot>
    </v-sk-surface>
</template>
    
<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
    name: "VSurface",
    props: {
        width: {
            type: Number as PropType<number>,
            required: true
        },
        height: {
            type: Number as PropType<number>,
            required: true
        },
    },
});
</script>
    