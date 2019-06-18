<template>
  <v-container >
    <v-layout row wrap>

      <v-flex xs12>
        <h1 class="display-1 font-weight-medium">
          Gallery
        </h1>
        <p class="font-18 font-weight-regular">
          All your media organized and easy to find
        </p>
      </v-flex>

      <v-flex xs12 class="mt-4">
        <p class="font-18 font-weight-regular">
          Bloom music allows you to access all your photos and videos from any device.
        </p>
      </v-flex>

      <v-flex xs12 class="mt-4">
        <h3 class="headline">How to add media to your Bloom Gallery?</h3>
        <p class="font-18 font-weight-regular">
          To add media to your Bloom Gallery, you have to upload them to any folder of your
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
