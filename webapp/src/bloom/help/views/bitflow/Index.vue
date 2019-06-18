<template>
  <v-container >
    <v-layout row wrap>

      <v-flex xs12>
        <h1 class="display-1 font-weight-medium">
          Bitflow
        </h1>
        <p class="font-18 font-weight-regular">
          Download torrents to a secured cloud. Fast. Online.
        </p>
      </v-flex>

      <v-flex xs12 class="mt-4">
        <p class="font-18 font-weight-regular">
          Bitflow allows you to download torrents and HTTP(s) files directly in your Drive, and access them from any device.
        </p>
      </v-flex>

      <v-flex xs12 class="mt-4">
        <h3 class="headline">What is the max downlaod size?</h3>
        <p class="font-18 font-weight-regular">
          Currently, the max size for a downlaod is 5GB.
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
