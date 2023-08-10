<template>
    <v-sk-group :x="x" :y="y" :color="color" :style="typeof style === 'string' ? style : undefined"
        :strokeWidth="strokeWidth" :invertClip="invertClip">
        <slot></slot>
        <v-sk-group-clip>
            <slot name="clip"></slot>
        </v-sk-group-clip>
    </v-sk-group>
</template>
    
<script lang="ts">
import { PropType, defineComponent } from "vue";

export default defineComponent({
    name: "VGroup",
    props: {
        x: {
            type: Number as PropType<number>,
            required: false
        },
        y: {
            type: Number as PropType<number>,
            required: false
        },
        color: {
            type: String as PropType<string>,
            required: false
        },
        style: {
            type: String as PropType<'fill' | 'stroke'>,
            required: false
        },
        strokeWidth: {
            type: Number as PropType<number>,
            required: false
        },
        invertClip: {
            type: Boolean as PropType<boolean>,
            required: false
        },
    },
});
</script>
    