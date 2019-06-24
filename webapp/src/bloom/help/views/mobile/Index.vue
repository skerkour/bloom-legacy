<template>
  <div>
    <v-container class="main-container">
      <v-layout row wrap>

        <v-flex xs12>
          <h1 class="display-1 font-weight-medium">
            Mobile Apps
          </h1>
          <p class="font-18 font-weight-regular mt-4">
            Currently only the Android app is available:
            <br/>
            <br/>
            <a href="https://play.google.com/store/apps/details?id=com.bloom42.bloomx" target="_blank" rel="noopener">
              <img alt="Get it on Google Play" src="/kernel/static/imgs/android_badge_generic.png" height="84"/>
            </a>
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
