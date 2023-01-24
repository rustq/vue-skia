<template>
  <h1>Demo</h1>
  <p>This is a demo</p>
  <v-surface width="400" height="500">
    <v-layer>
      <v-circle x="80" y="180" r="150" fill="ffffff" stroke="ff00ff"/>
      <HelloWorld />
    </v-layer>
  </v-surface>
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