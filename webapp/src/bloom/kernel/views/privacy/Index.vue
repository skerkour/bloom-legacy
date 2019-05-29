<template>
  <div>
  <v-container class="mt-4 text-xs-center">
    <v-layout row wrap justify-center>

      <v-flex xs12 class="text-xs-center mb-4">
        <h1 class="display-1 font-weight-medium">
          Privacy policy
        </h1>
      </v-flex>

      <v-flex xs12 sm10 md8 class="text-xs-left mb-4">
        <v-card class="elevation-0">
          <v-card-text>
            <p class="font-18 font-weight-regular">
              TODO...
            </p>
          </v-card-text>
        </v-card>
      </v-flex>


    </v-layout>
  </v-container>
  <blm-footer />
</div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


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

.font-5 {
  font-size: 15px;
}
</style>
