<template>
  <v-container >
    <v-layout row wrap>

      <v-flex xs12>
        <h1 class="display-1 font-weight-medium">
          Music
        </h1>
        <p class="font-18 font-weight-regular">
          A world of emotion. Available wherever you are.
        </p>
      </v-flex>

      <v-flex xs12 class="mt-4">
        <p class="font-18 font-weight-regular">
          Bloom music allows you to play any music in your Bloom drive from any device.
        </p>
      </v-flex>

      <v-flex xs12 class="mt-4">
        <h3 class="headline">How to add musics to your Bloom Music library?</h3>
        <p class="font-18 font-weight-regular">
          To add musics to your Bloom Music library, you need to upload your musics to any folder of your
          Bloom Drive.
        </p>
      </v-flex>

    </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
a {
  text-decoration: none;
}

.font-18 {
  font-size: 18px;
}
</style>
