<template>
  <v-container class="mt-3 text-center">
    <v-row class="justify-center align-center">

      <v-col cols="12" sm="6" class="text-center">
        <h1 class="display-3 mb-5">{{ $t('landing.mission') }}</h1>

        <!-- <h4 class="headline mt-5 font-weight-light">{{ $t('landing.subtitle1') }}</h4> -->
        <h4 class="headline mb-5 font-weight-light" v-html="$t('landing.subtitle2')"></h4>

        <blm-download-btn />

        <blm-other-downloads-link />
      </v-col>

      <v-col cols="12" sm="6" class="text-center mb-5">
        <!-- <img alt="hero" src="/static/imgs/landing/hero.png" height="480" /> -->
        <v-img alt="hero" src="/static/imgs/landing/hero.png" contain>
          <template v-slot:placeholder>
            <v-row
              class="fill-height ma-0"
              align="center"
              justify="center"
            >
              <v-progress-circular indeterminate color="grey lighten-2" />
            </v-row>
          </template>
        </v-img>
      </v-col>

    </v-row>

    <v-row class="mb-5 blm-features align-center">
      <v-col cols="12" sm="6" order="last" order-sm="first" class="text-center mb-5 mt-5">
        <img alt="secure" src="/static/imgs/landing/secure.svg" height="200" />
      </v-col>

      <v-col cols="12" sm="6">
        <h3 class="display-1 font-weight-regular">
          {{ $t('landing.feature1.title') }}
        </h3>
        <p class="mt-5" v-html="$t('landing.feature1.description')"></p>
      </v-col>
    </v-row>


    <v-row class="align-center">
      <v-col cols="12" sm="6">
        <h3 class="display-1 font-weight-regular">
          {{ $t('landing.feature2.title') }}
        </h3>
        <p class="mt-5" v-html="$t('landing.feature2.description')"></p>
      </v-col>

      <v-col cols="12" sm="6" class="text-center mb-5 mt-5">
        <img alt="groups" src="/static/imgs/landing/groups.svg" height="200" />
      </v-col>
    </v-row>


    <v-row class="justify-center mt-5">
      <v-col cols="12" class="text-center mb-5 mt-5">
        <v-btn to="/features" outlined x-large color="primary">
          {{ $t('landing.explore_features') }}
        </v-btn>
      </v-col>
    </v-row>

  </v-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import DownloadBtn from '@/components/coming_soon_btn.vue';
import OtherDownloadsLink from '@/components/other_downloads_link.vue';


@Component({
  components: {
    'blm-download-btn': DownloadBtn,
    'blm-other-downloads-link': OtherDownloadsLink,
  },
})
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
.blm-features {
  margin-top: 42px;
}
</style>
