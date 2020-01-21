<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 class="display-3 mb-5">{{ $t('faq.title') }}</h1>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-left m-3">
        <div class="mb-5 mt-5" v-for="question in questions" :key="question.question">
          <h2 class="display-2 mb-2">{{ question.question }}</h2>
          <p v-html="question.answer"></p>
          <v-divider />
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
      question: 'How are my data secured?',
      answer: `Some of your most personal moments are shared on WhatsApp, which is why we built end-to-end encryption.
      We use state of the art cryptography (
        <a href="https://tools.ietf.org/html/draft-irtf-cfrg-xchacha-03" target="_blank" rel="noopener noreferer">XChaCha20Poly1305</a>,
        <a href="https://en.wikipedia.org/wiki/Argon2" target="_blank" rel="noopener noreferer">Argon2id</a>,
        <a href="https://en.wikipedia.org/wiki/BLAKE_(hash_function)" target="_blank" rel="noopener noreferer">Blake2b</a>)
        to securely store your all your data.
      Your data are encrypted on your own devices and (even if we wanted) we can't read them.`,
    },
    {
      question: 'Where the encrpyted data are stored?',
      answer: 'The encrypted data are stored in a Scaleway Datacenter in France.',
    },
    {
      question: 'Why Bloom is more ethical than other service providers?',
      answer: 'Bloom is <b>100% Open Source</b>. Anybody can audit the can -> virtue',
    },
    {
      question: 'Why do I need to pay to store my data while $BigCompany Drive is (almost) free?',
      answer: `When you don't pay for a service, YOU are the product. It means that your personal data are either shared
      with third party advertisers, or algortihms are applied to your data to sell you more and more products
      you don't need. On the other side, on Bloom you data are encrypted and are not at the mercy of a tyranic government,
      a rogue employee, or an over jealous conojoint.`,
    },
    {
      question: 'I had a very poor experience with $BigCompany customer support, is yours better?',
      answer: 'By being a human scale company, and we do our best to respond to any customer request as soon as we can.',
    },
    {
      question: 'Do you issue refunds?',
      answer: `We do not issue refunds, but we offer a free tier
      to allow you to test our applications.`,
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
