<template>
  <v-container class="mt-3 text-center">
    <h1>FAQ</h1>

    <!-- How are my data secured?
    Parler de crpytographie, xchacha, argon2, blake2b
    -->


    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
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
