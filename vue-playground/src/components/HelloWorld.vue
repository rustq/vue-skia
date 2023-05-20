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
    </v-surface>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import launch from '../../../vue-skia-framework/launch';

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
