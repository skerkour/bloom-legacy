<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <h1 class="display-3 mb-5">
          <span class="blm-rounded-elegance">Bloom</span> is on a mission to empower people
          with open technologies
        </h1>

        <blm-download-btn />

        <p class="subtitle-1 mt-5"><router-link to="/download" class="text--darken-1 grey--text">
          Other downloads
        </router-link></p>
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
        <v-avatar size="128px" v-if="feature.icon">
          <v-img :src="feature.icon" contain />
        </v-avatar>
        <v-img v-else-if="feature.img" :src="feature.img" contain height="200px"/>
      </v-col>

      <v-col cols="12" sm="6" v-if="feature.name !== 'dl_btn'">
        <h3 class="display-1 font-weight-regular">
          {{ feature.name }}
        </h3>
        <p class="mt-5" v-html="feature.description"></p>
        <v-btn v-if="feature.learnMore" :to="`/features/${feature.learnMore}`" text color="primary">
          Learn more
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
export default class Features extends Vue {
  // props
  // data
  features = [
    {
      name: 'Drive',
      icon: '/static/imgs/icons/drive.svg',
      description: `A safe place for all your files. Bloom Drive is end to end encrypted.<br />
        <b>30GB free.</b>`,
      learnMore: 'drive',
    },
    {
      name: 'Music',
      icon: '/static/imgs/icons/music.svg',
      description: 'Never stop enjoying your music! Directly from your drive.',
      learnMore: 'music',
    },
    {
      name: 'Bitflow',
      icon: '/static/imgs/icons/bitflow.svg',
      description: `Your cloud-based download manager.
      Download any file and torrent directly to your Drive.`,
      learnMore: 'bitflow',
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
      name: 'Calendar',
      icon: '/static/imgs/icons/calendar.svg',
      description: 'TODO',
      learnMore: 'calendar',
    },
    {
      name: 'Chat',
      icon: '/static/imgs/icons/chat.svg',
      description: 'TODO',
      learnMore: 'chat',
    },
    {
      name: 'Books',
      icon: '/static/imgs/icons/books.svg',
      description: 'TODO',
      learnMore: 'books',
    },
    {
      name: 'Gallery',
      icon: '/static/imgs/icons/gallery.svg',
      description: 'Bloom Gallery is the home for all your photos and videos, organized and beautifully presented.',
      learnMore: 'gallery',
    },
    { name: 'dl_btn' },
    {
      name: 'Contacts',
      icon: '/static/imgs/icons/contacts.svg',
      description: 'Keep your contacts organized and up to date.',
      learnMore: 'contacts',
    },
    {
      name: 'Arcade',
      icon: '/static/imgs/icons/arcade.svg',
      description: 'Never stop having fun!',
      learnMore: 'arcade',
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
</style>
