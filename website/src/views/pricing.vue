<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="10" class="text-center mt-5">
        <h1 class="display-3 mb-5">Pricing</h1>
        <v-row class="justify-center">
          <v-col cols="12" md="3" class="text-center mt-5">
            <v-card class="mx-auto" outlined>
              <v-card-title class="text-center justify-center">Free</v-card-title>
              <v-card-text>
                blah blah blah
              </v-card-text>
            </v-card>
          </v-col>

          <v-col cols="12" md="3" class="text-center mt-5">
            <v-card class="mx-auto" outlined>
              <v-card-title class="text-center justify-center">Basic</v-card-title>
              <v-card-text>
                blah blah blah
              </v-card-text>
            </v-card>
          </v-col>

          <v-col cols="12" md="3" class="text-center mt-5">
            <v-card class="mx-auto" outlined>
              <v-card-title class="text-center justify-center">Pro</v-card-title>
              <v-card-text>
                blah blah blah
              </v-card-text>
            </v-card>
          </v-col>

          <v-col cols="12" md="3" class="text-center mt-5">
            <v-card class="mx-auto" outlined>
              <v-card-title class="text-center justify-center">Ultra</v-card-title>
              <v-card-text>
                blah blah blah
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <blm-download-btn />
        <blm-other-downloads-link />
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <h1 class="display-3 mb-5">Save big with Bloom</h1>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <h1 class="display-3 mb-5">Having questions?</h1>
        <div class="mt-5 pt-5">
          <router-link outlined to="/faq" >Explore the FAQ</router-link>
        </div>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center">
        <blm-download-btn />
        <blm-other-downloads-link />
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
export default class Pricing extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style scoped lang="scss">
</style>
