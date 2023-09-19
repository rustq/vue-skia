export default `<v-surface :width="360" :height="360">
  <v-rect :x="10" :y="220" :width="30" :height="30" color="cyan" :style="'fill'" />
  <v-line :strokeWidth="8" color="black" :p1="[100, 260]" :p2="[50, 285]" />
  <v-round-rect :x="220" :y="50" :width="80" :height="80" :r="10" color="fuchsia" :style="'stroke'" />
  <v-points :points="[
    [138, 10],
    [178, 90],
    [266, 103],
    [202, 165],
    [217, 254],
    [138, 212],
    [59, 254],
    [74, 165],
    [10, 103],
    [98, 90],
    [138, 10],
  ]" :style="'fill'" :strokeWidth="1" :color="'rgba(200, 255, 0, 0.7)'" />
  <v-image
    :x="0"
    :y="0"
    :image="'https://raw.githubusercontent.com/rustq/vue-skia/main/vue-playground/src/assets/logo.png'"
    :width="70"
    :height="70"
  ></v-image>
  <v-group :x="200" :y="160" color="violet" :style="'stroke'" :invertClip="true">
    <template #clip>
      <v-circle :cx="8" :cy="68" :r="40" />
    </template>
    <v-circle :cx="0" :cy="60" :r="50" :style="'fill'" />
    <v-circle :cx="0" :cy="60" :r="70" />
    <v-text :x="70" :y="0" :fontSize="16" color="red" text="Hello"></v-text>
  </v-group>
</v-surface>
`;
