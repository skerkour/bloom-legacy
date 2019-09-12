<template>
  <div>
    <v-container class="main-container">
      <v-layout row wrap>

        <v-flex xs12>
          <h1 class="display-1 font-weight-medium">
            Drive
          </h1>
          <p class="font-18 font-weight-regular">
            A safe place for all your files
          </p>
        </v-flex>

        <v-flex xs12 class="mt-4">
          <p class="font-18 font-weight-regular">
            Bloom Drive allows you to store your files in the cloud and to sync them across all your devices.
          </p>
        </v-flex>



        <v-flex xs12 class="mt-4">
          <h3 class="headline">What is the max size for a file to upload?</h3>
          <p class="font-18 font-weight-regular">
            Currently, the max size for a file to upload is 5GB.
          </p>
        </v-flex>

      </v-layout>
    </v-container>

    <blm-footer />
  </div>
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

.main-container {
  min-height: calc(100vh - 465px);
}
</style>
