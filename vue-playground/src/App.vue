<template>
  <main style="text-align: center">
    <template v-if="true">
      <h1>Vue Skia</h1>
      <p class="description">
        The <em>vue-skia</em> is a skia-based 2d graphics vue rendering library.
        It is based on Rust to implement software rasterization to perform
        rendering. It takes up less memory than the native canvas, however it is
        still a experiment project. And it's based entirely on vue syntax.
      </p>
      <p class="description">
        This super cool editor is based on <em>vue-live</em> !
      </p>
      <div class="livebox" v-if="!debug">
        <div class="hint">
          You can edit <a title="copy code to clipboard" @click="copy">this</a>
          <span>-></span>
        </div>
        <VueLive
          :editorProps="{ lineNumbers: true }"
          :code="!loading && !debug ? code : LoadingCode"
          :layout="CustomLayout"
          :components="{
            VSurface,
            VGroup,
            VRect,
            VCircle,
            VRoundRect,
            VLine,
            VPoints,
            VImage,
          }"
          @error="(e: any) => void 0"
          @input="input"
        />
      </div>
    </template>
    <template v-if="!loading && debug">
      <v-surface :width="400" :height="400">
        <v-points
          :points="[
            [128, 0],
            [168, 80],
            [256, 93],
            [192, 155],
            [207, 244],
            [128, 202],
            [49, 244],
            [64, 155],
            [0, 93],
            [88, 80],
            [128, 0],
          ]"
          :style="'fill'"
          :strokeWidth="1"
          :color="'rgba(200, 255, 0, 1)'"
        >
        </v-points>
        <v-circle
          :cx="200"
          :cy="260"
          :r="80"
          :style="'stroke'"
          color="#ee22ee"
        />
        <v-rect
          :x="10"
          :y="220"
          :width="30"
          :height="30"
          color="#00aaff"
          :style="'fill'"
        >
        </v-rect>
        <v-image
          :x="0"
          :y="0"
          :image="'https://raw.githubusercontent.com/rustq/vue-skia/main/vue-playground/src/assets/logo.png'"
          :width="70"
          :height="70"
        ></v-image>
      </v-surface>
    </template>
    <github-corners
      href="https://github.com/rustq/vue-skia"
      gitColor="#FFFFFF"
    />
  </main>
</template>
<script lang="ts">
import { defineComponent, markRaw } from "vue";
import launch, {
  VSurface,
  VGroup,
  VRect,
  VCircle,
  VRoundRect,
  VLine,
  VPoints,
  VImage,
} from "vue-skia";
import { VueLive } from "vue-live";
import GithubCorners from "@uivjs/vue-github-corners";
import CustomLayout from "./CustomLayout.vue";
import code from "./code";
import LoadingCode from "./loading-code";
import "vue-live/style.css";
import "prism-themes/themes/prism-night-owl.css";
export default defineComponent({
  name: "App",
  components: {
    VueLive,
    GithubCorners,
    VSurface,
    VGroup,
    VRect,
    VCircle,
    VRoundRect,
    VLine,
    VPoints,
    VImage,
  },
  data() {
    return {
      CustomLayout: markRaw(CustomLayout),
      loading: true,
      count: 2,
      VSurface,
      VGroup,
      VRect,
      VCircle,
      VRoundRect,
      VLine,
      VPoints,
      VImage,
      code,
      LoadingCode,
      debug: false,
      error: undefined,
    };
  },
  mounted() {
    launch().then(() => {
      try {
        const code = decodeURIComponent(
          new URLSearchParams(location.search).get("code") || ""
        );

        if (code) {
          this.code = code;
        }
      } catch (error) {
        //
      }

      this.loading = false;
    });
  },
  methods: {
    input(event: any) {
      this.code = event.target._value;
    },
    copy() {
      try {
        const url = new URL(location.href);
        url.search = new URLSearchParams({
          code: encodeURIComponent(this.code),
        }).toString();

        const input = document.createElement("input");
        input.setAttribute("value", url.toString());
        document.body.appendChild(input);
        input.select();
        document.execCommand("copy");
        document.body.removeChild(input);

        alert("🥳 code copied");
      } catch (error) {
        //
      }
    },
  },
});
</script>

<style>
body {
  background-color: #ded;
}

.prism-editor-wrapper {
  background-color: #222;
  color: #eee;
  padding: 8px 12px;
  box-sizing: border-box;
}

.separate {
  display: flex;
  flex-direction: column;
  width: 950px;
  margin: 30px auto;
}

.preview-separate {
  padding: 30px;
  background-color: #fff;
  text-align: center;
  border-radius: 10px 10px 0 0;
}

.description {
  max-width: 600px;
  margin: 30px auto;
  text-align: left;
}

.livebox {
  position: relative;
  max-width: 1300px;
  margin: auto;
  font-size: 14px;
}

.hint {
  position: absolute;
  top: 100px;
  left: -200px;
  font-family: "Nanum Pen Script";
  font-size: 2em;
  color: rgb(0, 161, 132);
  transform: rotate(-30deg);
  transition: transform 0.2s;
}

@media (max-width: 1600px) {
  .hint {
    transform: none;
    top: -35px;
    left: 0;
  }

  .hint span {
    transform: rotate(80deg) translate(10px, 10px);
    display: inline-block;
  }

  .hint a {
    cursor: pointer;
    text-decoration: underline;
  }

  .separate {
    width: 90vw;
  }
}

.button-bar {
  height: 70px;
  padding: 5px 0;
  text-align: left;
}

.button-bar button {
  font-size: 1.5em;
  padding: 6px;
  border-radius: 8px;
  width: 200px;
}
</style>
