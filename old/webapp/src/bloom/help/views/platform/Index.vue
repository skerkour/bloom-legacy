<template>
  <div>
    <v-container class="main-container">
      <v-layout row wrap>

        <v-flex xs12>
          <h1 class="display-1 font-weight-medium">
            Platform
          </h1>
          <p class="font-18 font-weight-regular">
            The global cloud platform for developers & teams
          </p>
        </v-flex>

        <v-flex xs12 class="mt-4">
          <p class="font-18 font-weight-regular">
            Bloom platform provides the services you need to build, innovate, and scale your business.
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
