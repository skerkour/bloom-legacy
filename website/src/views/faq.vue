<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 class="display-3 mb-5">{{ $t('faq.title') }}</h1>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-left mt-3">
        <div class="mb-5" v-for="question in questions" :key="question.question">
          <h2 class="display-1 mb-2">{{ question.question }}</h2>
          <p v-html="question.answer"></p>
        </div>
      </v-col>

    </v-row>

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
  questions = [
    {
      question: 'Are my data secure?',
      answer: 'Oui lol',
    },
    {
      question: 'Are my data secure?2',
      answer: 'Oui lol',
    },
    {
      question: 'Are my data secure?3',
      answer: 'Oui lol',
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style scoped lang="scss">
</style>
