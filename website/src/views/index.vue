<template>
  <div>
    <v-container class="mt-3 text-center">
      <v-row row wrap justify-center>

        <v-col cols="12" sm="6" class="text-center">
          <h1 class="display-3 mb-5">Our mission: Empowering people</h1>

          <h4 class="headline mt-5 font-weight-light">Chat, Collaborate, Discover, Innovate.</h4>
          <h4 class="headline mb-5 font-weight-light">Take back control with
            <span class="blm-rounded-elegance"><b>Bloom</b></span>
          </h4>

          <blm-download-btn />

          <p class="subtitle-1 mt-5"><router-link to="/download" class="text--lighten grey--text">
            Other downloads
          </router-link></p>
        </v-col>

        <v-col cols="12" sm="6" class="text-center mb-5">
          <v-img alt="hero" src="/imgs/landing/hero.png" contain />
        </v-col>

      </v-row>
    </v-container>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Footer from '@/components/footer.vue';
import DownloadBtn from '@/components/download_btn.vue';


@Component({
  components: {
    'blm-footer': Footer,
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
