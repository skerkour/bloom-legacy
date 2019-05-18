<template>
<div v-if="$store.state.session">
  <v-container >
    <v-layout row>
      <v-flex xs12>
        <v-card class="elevation-0">
          <v-container
          fluid
          grid-list-md
          >
          <v-layout row wrap justify-left>
            <v-flex
            v-for="card in cards"
            xs4
            sm3
            lg2
            :key="card.title"
            >
            <v-tooltip bottom>
              <router-link :to="card.to" slot="activator">
                  <v-img
                  :src="card.src"
                  height="150px"
                  contain
                  class="app-icon"
                  >
                </v-img>

                  <p class="headline text-xs-center app-name mt-3">{{ card.name }}</p>
            </router-link>
            <span>{{ card.tooltip }}</span>
          </v-tooltip>
        </v-flex>
      </v-layout>
    </v-container>
  </v-card>
</v-flex>
</v-layout>
</v-container>
</div>
  <blm-landing v-else />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';


@Component({
  components: {
    'blm-landing': Landing,
  },
})
export default class Index extends Vue {
  private cards = [
    {
      name: 'Platform',
      src: '/kernel/static/imgs/logos/platform.svg',
      to: '/platform',
      tooltip: 'Global cloud services for developers',
    },
    {
      name: 'Drive',
      src: '/kernel/static/imgs/logos/drive.svg',
      to: '/drive',
      tooltip: 'Store your files securely in the cloud and access them everywhere',
    },
    {
      name: 'Bitflow',
      src: '/kernel/static/imgs/logos/bitflow.svg',
      to: '/bitflow',
      tooltip: 'Download manager in the cloud',
    },
    {
      name: 'Gallery',
      src: '/kernel/static/imgs/logos/gallery.svg',
      to: '/gallery',
      tooltip: 'Bloom Gallery is the home for all your photos and videos, \
        organized and easy to share.',
    },
    {
      name: 'Music',
      src: '/kernel/static/imgs/logos/music.svg',
      to: '/music',
      tooltip: 'Bloom Music is the home for all your music. \
        Play music without interruption, online or off.',
    },
    {
      name: 'Contacts',
      src: '/kernel/static/imgs/logos/contacts.svg',
      to: '/contacts',
      tooltip: 'Keep your contacts organized and up to date',
    },
    {
      name: 'Arcade',
      src: '/kernel/static/imgs/logos/arcade.svg',
      to: '/arcade',
      tooltip: 'Never stop having fun',
    },
    {
      name: 'Calendar',
      src: '/kernel/static/imgs/logos/calendar.svg',
      to: '/calendar',
      tooltip: 'Get organized on the go with the Bloom Calendar',
    },
    {
      name: 'Notes',
      src: '/kernel/static/imgs/logos/notes.svg',
      to: '/notes',
      tooltip: 'Quickly capture what\'s on your mind',
    },
    {
      name: 'MyAccount',
      src: '/kernel/static/imgs/logos/myaccount.svg',
      to: '/myaccount',
      tooltip: 'Manage and secure your Bloom account',
    },
  ];
}
</script>


<style scoped lang="scss">
.v-card {
  border-radius: 8px;
}

.v-card__actions p {
  width: 100%;
}

a {
  text-decoration: none;
}

.app-icon {
  border-radius: 28px;
}

.app-name {
  color: #787878;
}
</style>
