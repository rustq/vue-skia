export default `<v-surface :width="400" :height="400">
    <v-points :points="[
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
    ]" :style="'fill'" :strokeWidth="1" :color="'rgba(200, 255, 0, 0.7)'" />
  <v-circle :cx="200" :cy="260" :r="80" :style="'stroke'" color="#ee22ee" />
  <v-rect :x="10" :y="220" :width="30" :height="30" color="#00aaff"
    :style="'fill'" />
  <v-round-rect :x="220" :y="50" :width="80" :height="80" :r="10" color="#ee22ee" :style="'stroke'" />
  <v-points :style="'fill'" :strokeWidth="2" color="#00aaff" :points="[
      [100, 260],
      [80, 300],
      [120, 300],
    ]" />
</v-surface>
`