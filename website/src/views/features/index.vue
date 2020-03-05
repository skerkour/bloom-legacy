<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center">
        <h1 :class="{'display-1': $vuetify.breakpoint.smAndDown,
        'display-3': $vuetify.breakpoint. mdAndUp, 'mb-5': true}">
          {{ pain }}
        </h1>
        <h4 :class="{'headline': $vuetify.breakpoint.smAndDown,
        'display-1': $vuetify.breakpoint. mdAndUp}" class="mb-3">{{ $t('features.solution') }}</h4>
        <h5 :class="{'headline': $vuetify.breakpoint.smAndDown,
        'display-1': $vuetify.breakpoint. mdAndUp}"
          class="font-weight-regular">{{ $t('features.title') }}</h5>

        <blm-download-btn class="mt-5" />

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
  // computed
  get pain(): any {
    const pains = [this.$t('labels.pain1'), this.$t('labels.pain2'), this.$t('labels.pain3'),
      this.$t('labels.pain4')];
    return pains[Math.floor(Math.random() * pains.length)];
  }

  get features(): any[] {
    return [
      // {
      //   name: 'Chat',
      //   icon: '/static/imgs/icons/chat.svg',
      //   description: `Bloom messages and calls are always end-to-end encrypted and
      //   engineered to keep your communication safe. We can't read your messages or see your calls
      //   and no one else can either.`,
      //   learnMore: 'chat',
      // },
      {
        name: 'Drive',
        icon: '/static/imgs/icons/drive.svg',
        description: this.$t('features.drive_description'),
        learnMore: 'drive',
      },
      // {
      //   name: 'Music',
      //   icon: '/static/imgs/icons/music.svg',
      //   description: this.$t('features.music_description'),
      //   learnMore: 'music',
      // },
      {
        name: 'Calendar',
        icon: '/static/imgs/icons/calendar.svg',
        description: this.$t('features.calendar_description'),
        learnMore: 'calendar',
      },
      {
        name: 'Notes',
        icon: '/static/imgs/icons/notes.svg',
        description: this.$t('features.notes_description'),
        learnMore: 'notes',
      },
      {
        name: 'Contacts',
        icon: '/static/imgs/icons/contacts.svg',
        description: this.$t('features.contacts_description'),
        learnMore: 'contacts',
      },
      { name: 'dl_btn' },
      {
        name: 'Bitflow',
        icon: '/static/imgs/icons/bitflow.svg',
        description: this.$t('features.bitflow_description'),
        learnMore: 'bitflow',
      },
      {
        name: 'Arcade',
        icon: '/static/imgs/icons/arcade.svg',
        description: this.$t('features.arcade_description'),
        learnMore: 'arcade',
      },
      // {
      //   name: 'Books',
      //   icon: '/static/imgs/icons/books.svg',
      //   description: 'Bloom Books is
      // the single destination for all the books you love, and the ones you\'re about to. ',
      //   learnMore: 'books',
      // },
      // { name: 'dl_btn' },
      // {
      //   name: 'Gallery',
      //   icon: '/static/imgs/icons/gallery.svg',
      //   description: this.$t('features.gallery_description'),
      //   learnMore: 'gallery',
      // },
      {
        name: 'QR Codes',
        icon: '/static/imgs/icons/qrcode.svg',
        description: this.$t('features.qrcodes_description'),
      },
      { name: 'dl_btn' },
    ];
  }
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

.v-btn--router {
  color: white !important;
}
</style>
