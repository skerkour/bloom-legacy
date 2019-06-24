<template>
  <div>
    <v-container class="main-container">
      <v-layout row wrap>

        <v-flex xs12>
          <h1 class="display-1 font-weight-medium">
            Phaser
          </h1>
          <p class="font-18 font-weight-regular">
            Continuous security made easy
          </p>
        </v-flex>

        <v-flex xs12 class="mt-4">
          <p class="font-18 font-weight-regular">
            Phaser is a cloud (and open source) security scanner for networks and websites.
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
