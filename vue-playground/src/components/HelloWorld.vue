<template>
  <div class="hello">
    <h2>
      {{ msg }}<button @click="count += 1">+</button
      ><button @click="count -= 1">-</button>
    </h2>
    <p v-if="loading">wasm loading</p>
    <v-surface v-if="!loading" :width="600" :height="400">
      <template v-if="count >= 0">
        <template
          :key="70 + index"
          v-for="(_, index) in new Array(count).fill(true)"
        >
          <v-rect
            :x="(index % 7) * 10 + 100"
            :y="(index % 3) * 10 + 100"
            :width="200"
            :height="200"
            :a="100"
            :r="index % 5 ? 0 : 200"
            :g="255"
            :b="0"
          />
        </template>
      </template>
      <template v-if="10 - count >= 0">
        <template
          :key="100 + index"
          v-for="(_, index) in new Array(10 - count).fill(true)"
        >
          <v-circle
            :cx="(index % 7) * 10 + 300"
            :cy="200"
            :r="10 + (count >= 7 && true ? 30 : index)"
            :a="100"
            :g="50"
            :b="200"
          />
        </template>
      </template>
      <v-roundRect
        :x="120"
        :y="70"
        :r="10"
        :width="100"
        :height="60"
        :a="100"
        :g="0"
        :b="10"
      ></v-roundRect>
      <v-line
        :p1="[130, 30]"
        :p2="[230, 130]"
        :strokeWidth="4"
        :r="10"
        :g="50"
        :b="200"
        :a="100"
      ></v-line>
      <v-points
        :points="[
          [260, 190],
          [250, 240],
          [210, 300],
          [150, 170],
          [170, 120],
        ]"
        :strokeWidth="4"
        :r="200"
        :g="50"
        :b="200"
        :a="100"
      ></v-points>
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
        :strokeWidth="4"
        :r="20"
        :g="20"
        :b="250"
        :a="100"
      ></v-points>
    </v-surface>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import launch from "../../../vue-skia-framework/launch";

export default defineComponent({
  name: "HelloWorld",
  props: {
    msg: String,
  },
  data() {
    return {
      loading: true,
      count: 2,
    };
  },
  mounted() {
    launch().then(() => {
      this.loading = false;
    });
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
