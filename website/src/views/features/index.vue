<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <h1 class="display-3 mb-5">
          {{ $t('features.title') }}
        </h1>

        <blm-download-btn />

        <blm-other-downloads-link />
      </v-col>
    </v-row>

    <v-row class="blm-features"></v-row>

    <v-row class="justify-center mb-5 mt-5 align-center blm-feature"
      v-for="(feature, index) in features" :key="index">
      <v-col cols="12" v-if="feature.name === 'dl_btn'">
        <div style="height: 100px;">
          <blm-download-btn outlined />
        </div>
      </v-col>

      <v-col cols="12" sm="6" class="pt-2 pb-2" v-if="feature.name !== 'dl_btn'"
        order="first" :order-sm="index % 2 === 0 ? 'first' : 'last'">
        <router-link :to="`/features/${feature.learnMore}`">
          <v-avatar size="128px" v-if="feature.icon">
            <img :src="feature.icon" contain />
          </v-avatar>
          <img v-else-if="feature.img" :src="feature.img" contain height="200px"/>
        </router-link>
      </v-col>

      <v-col cols="12" sm="6" v-if="feature.name !== 'dl_btn'">
        <router-link :to="`/features/${feature.learnMore}`">
        <h3 class="display-1 font-weight-regular">
          {{ feature.name }}
        </h3>
        <p class="mt-5" v-html="feature.description"></p>
        <v-btn v-if="feature.learnMore" :to="`/features/${feature.learnMore}`" text color="primary">
          {{ $t('labels.learn_more') }}
        </v-btn>
        </router-link>
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
export default class Features extends Vue {
  // props
  // data
  features = [
    // {
    //   name: 'Chat',
    //   icon: '/static/imgs/icons/chat.svg',
    //   description: `Bloom messages and calls are always end-to-end encrypted and
    //   engineered to keep your communication safe. We can't read your messages or see your calls,
    //   and no one else can either.`,
    //   learnMore: 'chat',
    // },
    {
      name: 'Drive',
      icon: '/static/imgs/icons/drive.svg',
      description: `A safe place for all your files. Bloom Drive is end to end encrypted.<br />
        <b>5GB free.</b>`,
      learnMore: 'drive',
    },
    {
      name: 'Music',
      icon: '/static/imgs/icons/music.svg',
      description: 'Never stop enjoying your music! Directly from your drive.',
      learnMore: 'music',
    },
    {
      name: 'Calendar',
      icon: '/static/imgs/icons/calendar.svg',
      description: 'Get organized on the go with the Bloom Calendar!',
      learnMore: 'calendar',
    },
    {
      name: 'Notes',
      icon: '/static/imgs/icons/notes.svg',
      description: `Quickly capture what’s on your mind and get a reminder later
        at the right place or time.`,
      learnMore: 'notes',
    },
    { name: 'dl_btn' },
    {
      name: 'Bitflow',
      icon: '/static/imgs/icons/bitflow.svg',
      description: `Your cloud-based download manager.
      Download any file and torrent directly to your Drive.`,
      learnMore: 'bitflow',
    },
    {
      name: 'Arcade',
      icon: '/static/imgs/icons/arcade.svg',
      description: 'Never stop having fun!',
      learnMore: 'arcade',
    },
    // {
    //   name: 'Books',
    //   icon: '/static/imgs/icons/books.svg',
    //   description: 'Bloom Books is
    // the single destination for all the books you love, and the ones you\'re about to. ',
    //   learnMore: 'books',
    // },
    {
      name: 'Contacts',
      icon: '/static/imgs/icons/contacts.svg',
      description: 'Keep your contacts organized and up to date.',
      learnMore: 'contacts',
    },
    { name: 'dl_btn' },
    {
      name: 'Gallery',
      icon: '/static/imgs/icons/gallery.svg',
      description: 'Bloom Gallery is the home for all your photos and videos, organized and beautifully presented.',
      learnMore: 'gallery',
    },
    {
      name: 'Scan QR Codes',
      icon: '/static/imgs/icons/qrcode.svg',
      description: 'Bloom apps are the perfect tool to scan your QR codes anytime and anywhere.',
    },
  ];

  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style scoped lang="scss">
.blm-features {
  margin-top: 80px;
}

.blm-feature {
  margin-top: 42px;
}

a, a * {
  text-decoration: none !important;
  color: inherit !important;
}
</style>
