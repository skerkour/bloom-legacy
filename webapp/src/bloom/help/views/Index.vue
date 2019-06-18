<template>
  <v-container >
    <v-layout row wrap>

      <v-flex xs12 class="mb-5 mt-3">
        <h1 class="display-1 font-weight-medium">
          Welcome to Bloom's help and documentation center
        </h1>
      </v-flex>

      <v-flex xs12>
        <v-layout row wrap justify-left>
          <v-flex v-for="card in cards" xs4 sm3 lg2 :key="card.title">
            <v-tooltip bottom>
              <router-link :to="card.to" slot="activator">
                <v-img :src="card.src" height="100px" contain />
                <p class="headline text-xs-center app-name mt-1">{{ card.name }}</p>
              </router-link>
              <span>{{ card.tooltip }}</span>
            </v-tooltip>
          </v-flex>
        </v-layout>
      </v-flex>


      <v-flex xs12 class="mt-5 font-18">
        <p>If you need further help, please visit our <router-link to="/contact">Contact page</router-link>.</p>
      </v-flex>

    </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component({})
export default class Index extends Vue {
  // props
  // data
    private cards = [
    {
      name: 'Drive',
      src: '/kernel/static/imgs/logos/drive.svg',
      to: '/help/drive',
      tooltip: 'Store your files securely in the cloud and access them everywhere',
    },
    {
      name: 'Platform',
      src: '/kernel/static/imgs/logos/platform.svg',
      to: '/help/platform',
      tooltip: 'Global cloud services for developers',
    },
    {
      name: 'Phaser',
      src: '/kernel/static/imgs/logos/phaser.svg',
      to: '/help/phaser',
      tooltip: 'The cloud native security scanner',
    },
    {
      name: 'MyAccount',
      src: '/kernel/static/imgs/logos/myaccount.svg',
      to: '/help/myaccount',
      tooltip: 'Manage and secure your Bloom account',
    },
    {
      name: 'Bitflow',
      src: '/kernel/static/imgs/logos/bitflow.svg',
      to: '/help/bitflow',
      tooltip: 'Download manager in the cloud',
    },
    {
      name: 'Music',
      src: '/kernel/static/imgs/logos/music.svg',
      to: '/help/music',
      tooltip: 'Bloom Music is the home for all your music. \
        Play music without interruption, online or off.',
    },
    {
      name: 'Gallery',
      src: '/kernel/static/imgs/logos/gallery.svg',
      to: '/help/gallery',
      tooltip: 'Bloom Gallery is the home for all your photos and videos, \
        organized and easy to share.',
    },
    {
      name: 'Contacts',
      src: '/kernel/static/imgs/logos/contacts.svg',
      to: '/help/contacts',
      tooltip: 'Keep your contacts organized and up to date',
    },
    {
      name: 'Arcade',
      src: '/kernel/static/imgs/logos/arcade.svg',
      to: '/help/arcade',
      tooltip: 'Never stop having fun',
    },
    {
      name: 'Calendar',
      src: '/kernel/static/imgs/logos/calendar.svg',
      to: '/help/calendar',
      tooltip: 'Get organized on the go with the Bloom Calendar',
    },
    {
      name: 'Notes',
      src: '/kernel/static/imgs/logos/notes.svg',
      to: '/help/notes',
      tooltip: 'Quickly capture what\'s on your mind',
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
a {
  text-decoration: none;
}

.font-18 {
  font-size: 18px;
}

.app-name {
  color: #787878;
}

.theme--dark {
  .app-name {
    color: #fafafa;
  }
}
</style>
