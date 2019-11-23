<template>
    <v-container class="mt-3 text-center">
      <v-row class="justify-center">

        <v-col cols="12" sm="6" class="text-center">
          <h1 class="display-3 mb-5">Our mission: Empowering people</h1>

          <h4 class="headline mt-5 font-weight-light">Chat, Collaborate, Discover, Innovate.</h4>
          <h4 class="headline mb-5 font-weight-light">Take back control with
            <span class="blm-rounded-elegance"><b>Bloom</b></span>
          </h4>

          <blm-download-btn />

          <p class="subtitle-1 mt-5"><router-link to="/download" class="text--darken-1 grey--text">
            Other downloads
          </router-link></p>
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

      <v-row class="justify-center">

        <v-col cols="12" class="text-center mb-5 mt-5">
          <v-btn to="/features" outlined x-large color="primary">
            Explore Features
          </v-btn>
        </v-col>
      </v-row>
    </v-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import DownloadBtn from '@/components/download_btn.vue';


@Component({
  components: {
    'blm-download-btn': DownloadBtn,
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
