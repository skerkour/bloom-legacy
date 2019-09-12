<template>
  <v-toolbar color="primary" dark flat fixed clipped-left app class="blm-toolbar">
    <v-toolbar-side-icon v-if="(side_icon && $store.state.session) || (service === 'help')" @click.stop="side_icon_clicked">
      <v-icon>mdi-menu</v-icon>
    </v-toolbar-side-icon>
    <router-link to="/">
      <v-toolbar-title class="mr-5 align-cente">
        <span class="blm-toolbar-title">
          <span class="blm-toolbar-title-primary"><b>Bloom</b></span><span class="blm-toolbar-title-secondary" v-if="secondary_title">&nbsp;{{ secondary_title }}</span>&nbsp;<span class="blm-toolbar-beta">beta</span>
        </span>
      </v-toolbar-title>
    </router-link>

    <v-text-field
      v-if="display_searchbar && $store.state.session"
      :value="$store.state.search"
      flat
      small
      clearable
      solo-inverted
      hide-details
      prepend-inner-icon="mdi-magnify"
      :label="searchbar_label"
      class="hidden-sm-and-down"
      @input="search"
    ></v-text-field>


    <v-spacer></v-spacer>

      <v-menu v-model="menu_apps" :close-on-content-click="false" transition="slide-y-transition"
        class="hidden-xs-only" v-if="$store.state.session" left>
        <template #activator="{ on: menu_activator }">
          <v-tooltip bottom>
            <template #activator="{ on: tooltip_activator }">
              <v-btn class="hidden-xs-only" icon v-on="{ ...tooltip_activator, ...menu_activator }">
                <v-icon>mdi-apps</v-icon>
              </v-btn>
            </template>
            <span>Bloom apps</span>
          </v-tooltip>
        </template>

        <v-card id="blm-toolbar-apps-card">
          <v-card-text class="text-xs-center">
            <v-layout row wrap justify-left>
              <v-flex v-for="app in apps" xs4 :key="app.title" justify-center class="text-xs-center" @click="menu_apps = false">
                <v-tooltip bottom>
                  <router-link :to="app.to" slot="activator">
                    <img :src="app.logo" height="60px" width="60px" />
                  </router-link>
                  <span>{{ app.name }}</span>
                </v-tooltip>
              </v-flex>
            </v-layout>
          </v-card-text>

          <v-card-actions class="justify-center">
            <v-btn color="primary" flat to="/" @click="menu_apps = false">More</v-btn>
          </v-card-actions>

      </v-card>
    </v-menu>


    <v-tooltip bottom>
      <v-btn slot="activator" icon to="/help">
        <v-icon>mdi-help-circle</v-icon>
      </v-btn>
      <span>Help</span>
    </v-tooltip>

    <v-menu
      v-model="menu"
      :close-on-content-click="false"
      transition="slide-y-transition"
      class="hidden-xs-only"
      v-if="$store.state.session"
      left
      >
      <v-btn icon slot="activator">
        <v-avatar size="40">
          <img v-if="$store.state.account" :src="$store.state.account.avatar_url" alt="Avatar">
          <img v-else src="/kernel/static/imgs/profile.jpg" alt="Avatar">
        </v-avatar>
      </v-btn>

      <v-card>
        <v-list>

          <v-list-tile avatar>

            <v-list-tile-avatar>
              <img v-if="$store.state.account" :src="$store.state.account.avatar_url" alt="Avatar">
              <img v-else src="/kernel/static/imgs/profile.jpg" alt="Avatar">
            </v-list-tile-avatar>

            <v-list-tile-content>
              <v-list-tile-title>
                <span v-if="$store.state.account">
                  {{ $store.state.account.display_name }}
                </span>
                <span v-else>Display Name</span>
              </v-list-tile-title>
              <v-list-tile-sub-title >
                <span v-if="$store.state.account">{{ $store.state.account.username }}</span>
                <span v-else>Username</span>
              </v-list-tile-sub-title>
            </v-list-tile-content>

          </v-list-tile>


          <v-list-tile class="text-xs-center">
            <v-btn small
              color="primary"
              to="/myaccount"
              id="menu-account"
              class="elevation-0"
              @click="menu = false">
              My Account
            </v-btn>
          </v-list-tile>


        </v-list>
        <v-divider></v-divider>
        <div class="text-xs-center">
          <v-btn flat small @click="sign_out">Sign out</v-btn>
        </div>

      </v-card>
    </v-menu>
    <div v-else-if="$route.meta.onboarding !== true">
      <v-toolbar-items>

      <v-btn flat to="/sign-in" id="sign-in-btn">Sign in</v-btn>
      <v-btn outline dark to="/register" id="register-btn">Try it Free</v-btn>
    </v-toolbar-items>

    </div>

  </v-toolbar>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class Toolbar extends Vue {
  // props
  // data
  menu = false;
  menu_apps = false;
  side_icon = true;
  display_searchbar = false;
  searchbar_label = 'Search';
  secondary_title = '';
  service = '';
  apps = [
    {
      logo: '/kernel/static/imgs/logos/platform.svg',
      name: 'Platform',
      to: '/platform',
    },
    {
      logo: '/kernel/static/imgs/logos/drive.svg',
      name: 'Drive',
      to: '/drive',
    },
    {
      logo: '/kernel/static/imgs/logos/bitflow.svg',
      name: 'Bitflow',
      to: '/bitflow',
    },
    {
      logo: '/kernel/static/imgs/logos/music.svg',
      name: 'Music',
      to: '/music',
    },
    {
      logo: '/kernel/static/imgs/logos/gallery.svg',
      name: 'Gallery',
      to: '/gallery',
    },
    {
      logo: '/kernel/static/imgs/logos/contacts.svg',
      name: 'Contacts',
      to: '/contacts',
    },
    {
      logo: '/kernel/static/imgs/logos/arcade.svg',
      name: 'Arcade',
      to: '/arcade',
    },
    {
      logo: '/kernel/static/imgs/logos/calendar.svg',
      name: 'Calendar',
      to: '/calendar',
    },
    {
      logo: '/kernel/static/imgs/logos/notes.svg',
      name: 'Notes',
      to: '/notes',
    },
    {
      logo: '/kernel/static/imgs/logos/myaccount.svg',
      name: 'MyAccount',
      to: '/myaccount',
    },
  ];


  // computed
  // lifecycle
  created() {
    this.setup();
  }


  // watch
  @Watch('$route')
  on_route_change() {
    this.setup();
  }


  // methods
  sign_out() {
    api.sign_out();
  }

  side_icon_clicked() {
    this.$emit('click:side-icon');
  }

  setup() {
    this.service = this.$route.meta.service;
    switch (this.service) {
      case 'myaccount':
        this.secondary_title = 'MyAccount';
        this.side_icon = true;
        this.display_searchbar = false;
        break;
      case 'drive':
        this.secondary_title = 'Drive';
        this.side_icon = true;
        // this.display_searchbar = true;
        this.display_searchbar = false;
        this.searchbar_label = 'Search Drive';
        break;
      case 'platform':
        this.secondary_title = 'Platform';
        this.side_icon = true;
        this.display_searchbar = false;
        break;
      case 'bitflow':
        this.secondary_title = 'Bitflow';
        this.side_icon = true;
        this.display_searchbar = false;
        break;
      case 'notes':
        this.secondary_title = 'Notes';
        this.side_icon = true;
        this.display_searchbar = false;
        break;
      case 'contacts':
        this.secondary_title = 'Contacts';
        this.side_icon = false;
        this.display_searchbar = false;
        this.searchbar_label = 'Search Contacts';
        break;
      case 'calendar':
        this.secondary_title = 'Calendar';
        this.side_icon = false;
        this.display_searchbar = false;
        break;
      case 'arcade':
        this.secondary_title = 'Arcade';
        this.side_icon = false;
        this.display_searchbar = false;
        break;
      case 'gallery':
        this.secondary_title = 'Gallery';
        this.side_icon = true;
        this.display_searchbar = false;
        break;
      case 'music':
        this.secondary_title = 'Music';
        this.side_icon = true;
        this.display_searchbar = false;
        break;
      case 'admin':
        this.secondary_title = 'Admin';
        this.side_icon = true;
        this.display_searchbar = false;
        break;
      case 'help':
        this.secondary_title = 'Help';
        this.side_icon = true;
        this.display_searchbar = false;
        break;
      default:
        this.secondary_title = '';
        this.side_icon = false;
        this.display_searchbar = false;
        this.searchbar_label = '';
        break;
    }
    document.title = `Bloom ${this.secondary_title}`;
  }

  search(q: string) {
    this.$store.commit('set_search', q);
  }
}
</script>


<style scoped lang="scss">
.blm-toolbar-title-primary {
  font-family: rounded_elegance;
  font-size: unset;
}
.blm-toolbar-title {
  font-size: larger;

  .blm-toolbar-beta {
    font-size: 12px;
    vertical-align: top;
  }
}

.v-card {
  border-radius: 6px;
}

#blm-toolbar-apps-card {
  width: 300px;
}

.v-toolbar a {
  color: white;
  text-decoration: none;
  text-overflow: ellipsis;
  overflow: hidden;
}

#menu-account {
  margin-left: calc(50% - 44px);
}

.v-content {
  height: 100%;
}

#register-btn, #sign-in-btn {
  height: 36px;
}
</style>
