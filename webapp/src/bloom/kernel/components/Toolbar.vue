<template>
  <v-toolbar color="primary" dark flat fixed clipped-left app class="blm-toolbar">
    <v-toolbar-side-icon v-if="side_icon && $store.state.session" @click.stop="side_icon_clicked">
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

    <v-tooltip bottom v-if="$store.state.session">
      <v-btn slot="activator" icon to="/" class="hidden-xs-only">
        <v-icon>mdi-apps</v-icon>
      </v-btn>
      <span>Bloom apps</span>
    </v-tooltip>
    <v-tooltip bottom>
      <v-btn slot="activator" icon href="https://help.bloom.sh" target="_blank" rel="noopener" class="hidden-xs-only">
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
                  {{ $store.state.account.first_name }} {{ $store.state.account.last_name }}
                </span>
                <span v-else>Name</span>
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
  side_icon = true;
  display_searchbar = false;
  searchbar_label = 'Search';
  secondary_title = '';


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
    switch (this.$route.meta.service) {
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
      default:
        this.secondary_title = '';
        this.side_icon = false;
        this.display_searchbar = false;
        this.searchbar_label = '';
        break;
    }
    document.title = `Bloom ${this.secondary_title}`;
    if (!this.$route.meta.auth || this.$route.meta.auth.layout !== 'authenticated') {
      this.side_icon = false;
    }
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
