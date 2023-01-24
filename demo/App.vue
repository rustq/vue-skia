<template>
  <v-layer x="0" y="0" id="app">
    <HelloWorld />
    <v-circle x="120" y="220" />
  </v-layer>
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'

export default {
  name: 'App',
  components: {
    HelloWorld
  }
}
</script>