<template>
  <v-layer>
    <v-rect x="40" y="40" width="100" height="100" fill="000000" stroke="ffffff">
      <v-rect v-for="item in list" :x="20 + item * 50" :y="20 + item * 40" width="50" height="50" fill="00ffaa" stroke="000000">
      </v-rect>
    </v-rect>
    <v-round-rect x="220" y="50" width="80" height="80" radius="10" fill="ee22ee" stroke="ffffff" />
    <v-triangle x="20" y="220" width="150" height="100" fill="ffffff" stroke="00aaff" />
    <v-triangle x="100" y="260" width="120" height="120" fill="00aaff" />
  </v-layer>
</template>

<script>
export default {
  name: 'HelloWorld',
  data() {
    return {
        list: [0, 1, 2]
    }
  }
}
</script>