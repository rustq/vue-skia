<template >
  <v-sk-image v-if="loaded" :image="base64String" :x="x" :y="y" :width="width" :height="height" :blur="blur" :grayscale="grayscale" :brighten="brighten" :invert="invert" />
</template>

<script lang="ts">
import { defineComponent, getCurrentInstance, PropType } from "vue";
import { ComponentInternalInstanceWithSoftSkiaWASM } from "../type";

class ImageLoader {
  static caches: { [url: string]: string } = {};

  public static load(url: string): Promise<string> {
    return new Promise((resolve, reject) => {
      if (this.caches[url]) {
        return resolve(this.caches[url]);
      }
      if (url.startsWith("data:image/png;base64")) {
        const base64String = url.replace("data:image/png;base64,", "");
        this.caches[url] = base64String;
        return resolve(base64String);
      } else {
        fetch(url)
          .then((res) => res.blob())
          .then((blob) => {
            const reader = new FileReader();
            reader.onloadend = () => {
              const base64String = (reader.result as string).replace(
                "data:image/png;base64,",
                ""
              );
              this.caches[url] = base64String;
              return resolve(base64String);
            };
            reader.readAsDataURL(blob);
          })
          .catch(e => {
            reject(e)
          });
      }
    });
  }
}

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
    },
    blur: {
      type: Number as PropType<number>,
      required: false,
    },
    grayscale: {
      type: Boolean as PropType<boolean>,
      required: false,
    },
    brighten: {
      type: Number as PropType<number>,
      required: false,
    },
    invert: {
      type: Boolean as PropType<boolean>,
      required: false,
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
    ImageLoader.load(this.image).then(base64String => {
      this.base64String = base64String;
      this.loaded = true;
      this.reUpdateRoot(vm);
    })
  }
});
</script>
