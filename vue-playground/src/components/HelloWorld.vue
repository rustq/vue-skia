<template>
  <div class="hello">
    <h1>
      {{ msg }}<button @click="count += 1">+</button
      ><button @click="count -= 1">-</button>{{ count }}
    </h1>
    <p v-if="loading">wasm loading</p>
    <v-surface v-if="!loading">
      <template :key="index" v-for="(_, index) in new Array(count).fill(true)">
        <v-circle
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
    </v-surface>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HelloWorld",
  props: {
    msg: String,
  },
  data() {
    return {
      loading: true,
      count: 2,
    }
  },
  mounted() {
    const wasm = import("../../../wasm/pkg/wasm.js");
    wasm.then((v_sk: any) => {
      v_sk.default().then(() => {

        // @ts-ignore
        window.v_sk = v_sk;
        this.loading = false
      })
    })
  }
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
