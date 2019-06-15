<template>
  <div>
  <v-container class="mt-4 text-xs-center">
    <v-layout row wrap justify-center>
      <v-flex xs12 class="text-xs-center mb-3">
        <h1 class="display-1 font-weight-medium">
          Bloom Apps
        </h1>
      </v-flex>

      <v-flex xs12 sm10 md8 class="text-xs-center mb-5">
        <div class="elevation-0">
          <v-card-text>
            <p class="font-18 font-weight-regular">
              Currently only the Android app is available:
              <br/>
              <br/>
              <a href="https://play.google.com/store/apps/details?id=com.bloom42.bloomx" target="_blank" rel="noopener">
                <img alt="Get it on Google Play" src="/kernel/static/imgs/android_badge_generic.png" height="84"/>
              </a>
            </p>
          </v-card-text>
        </div>
      </v-flex>

      <v-flex xs12 class="text-xs-center mb-3 mt-3">
        <h1 class="display-1 font-weight-medium">
          Contribute
        </h1>
      </v-flex>

      <v-flex xs12 sm10 md8 class="text-xs-center mb-5">
        <div class="elevation-0">
          <v-card-text>
            <p class="font-18 font-weight-regular">
              If you have the skills and the will to build the Open source IOS App please contact us:
              <span><a href="mailto:hello@bloom.sh">hello@bloom.sh</a></span>
            </p>
          </v-card-text>
        </div>
      </v-flex>

    </v-layout>
  </v-container>

  <blm-footer />
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Contributors from '../../components/Contributors.vue';


@Component({
  components: {
    'blm-contributors': Contributors,
  },
})
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

.font-5 {
  font-size: 15px;
}
</style>
