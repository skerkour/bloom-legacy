<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 :class="{'display-1': $vuetify.breakpoint.smAndDown,
        'display-3': $vuetify.breakpoint. mdAndUp, 'mb-5': $vuetify.breakpoint. mdAndUp}">
        {{ $t('faq.title') }}</h1>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-left m-3">
        <div class="mb-5 mt-5" v-for="(question, index) in questions" :key="index">
          <h2 class="display-1 font-weight-medium mb-2">{{ question.question }}</h2>
          <p v-html="question.answer"></p>
          <v-divider v-if="index !== questions.length - 1"/>
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
export default class FAQ extends Vue {
  // props
  // data
  get questions(): any[] {
    return [
      {
        question: this.$t('faq.q1'),
        answer: this.$t('faq.a1'),
      },
      {
        question: this.$t('faq.q2'),
        answer: this.$t('faq.a2'),
      },
      {
        question: this.$t('faq.q3'),
        answer: this.$t('faq.a3'),
      },
      {
        question: this.$t('faq.q4'),
        answer: this.$t('faq.a4'),
      },
      {
        question: this.$t('faq.q8'),
        answer: this.$t('faq.a8'),
      },
      {
        question: this.$t('faq.q5'),
        answer: this.$t('faq.a5'),
      },
      {
        question: this.$t('faq.q7'),
        answer: this.$t('faq.a7'),
      },
      {
        question: this.$t('faq.q6'),
        answer: this.$t('faq.a6'),
      },
    ];
  }

  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style scoped lang="scss">
</style>
