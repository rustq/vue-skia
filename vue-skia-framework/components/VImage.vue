<template >
  <v-sk-image v-if="loaded" :image="base64String" :x="x" :y="y" :width="width" :height="height" />
</template>

<script lang="ts">
import { defineComponent, getCurrentInstance, PropType } from "vue";
import { ComponentInternalInstanceWithSoftSkiaWASM } from "../type";

export default defineComponent({
  name: "VImage",
  data() {
    return {
      loaded: false,
      base64String: ""
    }
  },
  props: {
    image: {
      type: String as PropType<string>,
      required: true
    },
    x: {
      type: Number as PropType<number>,
      required: true,
    },
    y: {
      type: Number as PropType<number>,
      required: true,
    },
    width: {
      type: Number as PropType<number>,
      required: true,
    },
    height: {
      type: Number as PropType<number>,
      required: true,
    }
  },
  methods: {
    reUpdateRoot(vm: ComponentInternalInstanceWithSoftSkiaWASM) {
      const instance = vm;
      var parent = instance.parent as ComponentInternalInstanceWithSoftSkiaWASM;
      while (!("_ssw_id" in parent)) {
        parent = (parent as ComponentInternalInstanceWithSoftSkiaWASM).parent as ComponentInternalInstanceWithSoftSkiaWASM;
      }
      parent.update()
    }
  },
  mounted() {
    const vm = getCurrentInstance() as ComponentInternalInstanceWithSoftSkiaWASM;
    if (this.image?.startsWith("data:image/png;base64")) {
      const base64String = this.image.replace("data:image/png;base64,", "")
      this.base64String = base64String;
      this.loaded = true;
      this.reUpdateRoot(vm);
    } else {
      fetch(this.image)
        .then((res) => res.blob())
        .then((blob) => {
          const reader = new FileReader();
          reader.onloadend = () => {
            const base64String = (reader.result as string).replace("data:image/png;base64,", "");
            this.base64String = base64String;
            this.loaded = true;
            this.reUpdateRoot(vm);
          };
          reader.readAsDataURL(blob);
        });
    }
  }
});
</script>
