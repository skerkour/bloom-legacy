<template>
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center">

        <h1 class="mt-5 mb-3 display-1 font-weight-medium">Security</h1>
        <p class="body-1 font-weight-regular mb-5">
          Keeping our customer data safe and secure is our top priority. We take threats very
        seriously and work hard to protect our customers and their data. Your input and feedback
          on our security is greatly appreciated.
        </p>

        <h2 class="mt-5 mb-3 display-1 font-weight-medium">Reporting Issues</h2>

        <p class="body-1 font-weight-regular">
          Send reports directly to <a href="mailto:security@bloom.sh">security@bloom.sh</a>
          and we will get back to you faster than the light.
        </p>

      </v-col>
    </v-row>
  </v-container>
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
</style>
