<template>
  <div>
  <v-container class="mt-4 text-xs-center">
    <v-layout row wrap justify-center>
      <v-flex xs12 class="text-xs-center mb-3">
        <h1 class="display-1 font-weight-medium">
          Security
        </h1>
      </v-flex>

      <v-flex xs12 sm10 md8 class="text-xs-center mb-5">
        <v-card class="elevation-0">
          <v-card-text>
            <p class="font-18 font-weight-regular">
              Keeping our customer data safe and secure is our top priority. We take threats very
              seriously and work hard to protect our customers and their data. Your input and feedback
              on our security is greatly appreciated.
            </p>

            <h4 class="mt-5 mb-3 display-1 font-weight-medium">Reporting Issues</h4>

            <p class="font-18 font-weight-regular">
              Send reports directly to <a href="mailto:security@bloom.sh">security@bloom.sh</a>
              and we will get back to you faster than the light.
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
</style>
