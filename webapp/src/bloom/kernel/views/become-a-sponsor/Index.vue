<template>
  <div>
  <v-container class="mt-4 text-xs-center">
    <v-layout row wrap justify-center>
      <v-flex xs12 class="text-xs-center mb-3">
        <h1 class="display-1 font-weight-medium">
          How to become a sponsor?
        </h1>
      </v-flex>

      <v-flex xs12 sm10 md8 class="text-xs-center mb-3">
        <div class="elevation-0">
          <v-card-text>
            <p class="font-18 font-weight-regular">
              Bloom is an open source project that is completely free to use (<i>free</i> as in Freedom).
              We hope you enjoy using Bloom and if you want to support
              the team and the contributors you can help by backing us. <br />
              Any amount would help support development on this project and is greatly appreciated.<br/>
              If you want to have your logo on our
                <router-link to="/about#sponsors">about page</router-link>,
                feel free to email us: <a href="mailto:hello@bloom.sh">hello@bloom.sh</a>
            </p>
          </v-card-text>
        </div>
      </v-flex>

      <v-flex xs12 class="text-xs-center mb-4">
        <h1 class="display-1 font-weight-medium">
          One-Time Donation
        </h1>
      </v-flex>

      <v-flex xs12 class="text-xs-center mb-4 font-18">
        <a href="https://paypal.me/z0mbie42" target="_blank" rel="noopener">
          <img src="/kernel/static/imgs/paypal_donate.gif" height="42"/>
        </a>

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
  hover_elevation = 4;
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
