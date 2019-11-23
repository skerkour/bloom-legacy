<template>
  <v-container class="mt-3 text-center">
    <v-row class="justify-center align-center">

      <v-col cols="12" sm="6" class="text-center">
        <h1 class="display-3 mb-5">{{ $t('landing.mission') }}</h1>

        <h4 class="headline mt-5 font-weight-light">{{ $t('landing.subtitle1') }}</h4>
        <h4 class="headline mb-5 font-weight-light" v-html="$t('landing.subtitle2')"></h4>

        <blm-download-btn />

        <blm-other-downloads-link />
      </v-col>

      <v-col cols="12" sm="6" class="text-center mb-5">
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
        <v-img alt="hero" src="/static/imgs/landing/secure.svg" contain height="200px">
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

      <v-col cols="12" sm="6">
        <h3 class="display-1 font-weight-regular">
          Fast, Simple, Secure
        </h3>
        <p class="mt-5">
          As an Open Source project anyone can inspect how Bloom works.
          We use state of the art cryptography to keep your data secure.
          There are no ads, no affiliate marketers, no creepy tracking. <br />
          Just open technology for a fast, simple, and secure experience.
        </p>
      </v-col>
    </v-row>


    <v-row class="align-center">
      <v-col cols="12" sm="6">
        <h3 class="display-1 font-weight-regular">
          Bring your team together
        </h3>
        <p class="mt-5">
          Whether it's your friends or your colleagues, Bloom is made for collaboration.
          Organize conversations, share files and ideas, organize your life.
          Go from chatting to meeting up with Calendar, Notes, Drive and more.<br />
          Never stop having fun.
        </p>
      </v-col>

      <v-col cols="12" sm="6" class="text-center mb-5 mt-5">
        <v-img alt="hero" src="/static/imgs/landing/groups.svg" contain height="200px">
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
import DownloadBtn from '@/components/download_btn.vue';
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
