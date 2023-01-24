<template>
  <p>🤔</p>
  <v-rect x="100" :y="count * 300">
    <v-circle :size="count * 8" :x="19 - count" y="30" />
    <v-circle :x="20" y="130" />
  </v-rect>
  <v-rect x="120" :y="count * 200" />
  <v-rect x="20" :y="count * 100" />
  <v-rect x="120" :y="count * 80" />
</template>

<script>
export default {
  name: 'HelloWorld',
  data() {
    return {
        count: 1
    }
  }
}
</script>