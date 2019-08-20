<template>
  <div>
  <v-container class="mt-3 text-xs-center">
    <v-layout row wrap justify-center>

      <v-flex xs12 class="text-xs-center mb-5">
        <v-img alt="404" src="/kernel/static/imgs/404.svg" id="img-404" contain />
      </v-flex>

      <v-flex xs12 class="text-xs-center">
        <h1 class="display-2 font-weight-medium">
          Page not found
        </h1>
      </v-flex>

      <v-flex xs12 class="text-xs-center mt-4">
        <v-btn to="/" color="primary">Back to Home</v-btn>
      </v-flex>

    </v-layout>
  </v-container>
  <blm-footer />
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component({})
export default class Index extends Vue {
  // props
  // data
  hover_elevation = 4;
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style scoped lang="scss">
#img-404 {
  max-height: 50vh;
}

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
