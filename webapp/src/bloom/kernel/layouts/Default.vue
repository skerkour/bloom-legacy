<template>
  <div id="layout-wrapper">
    <v-navigation-drawer
      v-model="drawer"
      fixed
      clipped
      app
      width="250"
      v-if="this.$store.state.session || $route.meta.service === 'help'">

      <v-img class="hidden-lg-and-up pointer" :aspect-ratio="16/6"
      @click="goto('/myaccount'); drawer = false"
        src="/kernel/static/imgs/material.jpg">
        <v-layout pa-2 column fill-height class="lightbox white--text">
          <v-avatar>
            <img v-if="$store.state.account" :src="$store.state.account.avatar_url" alt="Avatar">
            <img v-else src="/kernel/static/imgs/profile.jpg" alt="Avatar">
          </v-avatar>
          <v-flex shrink>
            <div class="subheading">
              <span v-if="$store.state.account">
                {{ $store.state.account.first_name }} {{ $store.state.account.last_name }}
              </span>
              <span v-else>Name</span>
            </div>
            <div class="body-1">
              <span v-if="$store.state.account">{{ $store.state.account.username }}</span>
              <span v-else>Username</span>
            </div>
          </v-flex>
        </v-layout>
      </v-img>

      <blm-drive-drawer v-if="$route.meta.service === 'drive'" />
      <blm-bitflow-drawer v-else-if="$route.meta.service === 'bitflow'" />
      <blm-myaccount-drawer v-else-if="$route.meta.service === 'myaccount'" />
      <blm-platform-drawer v-else-if="$route.meta.service === 'platform'" />
      <blm-notes-drawer v-else-if="$route.meta.service === 'notes'" />
      <blm-gallery-drawer v-else-if="$route.meta.service === 'gallery'" />
      <blm-music-drawer v-else-if="$route.meta.service === 'music'" />
      <blm-admin-drawer v-else-if="$route.meta.service === 'admin'" />
      <blm-help-drawer v-else-if="$route.meta.service === 'help'" />


      <v-divider class="hidden-sm-and-up"></v-divider>

      <v-list-tile class="hidden-sm-and-up" to="/help">
        <v-list-tile-action>
          <v-icon color="grey darken-1">mdi-help-circle</v-icon>
        </v-list-tile-action>
        <v-list-tile-title class="grey--text text--darken-1">help</v-list-tile-title>
      </v-list-tile>


      <v-list-tile class="drawer-sign-out hidden-sm-and-up" @click="sign_out">
        <v-list-tile-action>
          <v-icon color="grey darken-1">mdi-power</v-icon>
        </v-list-tile-action>
        <v-list-tile-title class="grey--text text--darken-1">Sign out</v-list-tile-title>
      </v-list-tile>


    </v-navigation-drawer>


    <blm-toolbar @click:side-icon="drawer = !drawer" />


    <v-content>
      <!-- <v-container> -->
        <router-view></router-view>
      <!-- </v-container> -->
    </v-content>

  </div>
</template>

<script lang="ts">
import api from '@/bloom/kernel/api';
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import router from '@/bloom/kernel/router';
import { Route } from 'vue-router';
import DriveDrawer from '@/bloom/drive/components/Drawer.vue';
import BitflowDrawer from '@/bloom/bitflow/components/Drawer.vue';
import MyAccountDrawer from '@/bloom/myaccount/components/Drawer.vue';
import PlatformDrawer from '@/bloom/platform/components/Drawer.vue';
import MusicDrawer from '@/bloom/music/components/Drawer.vue';
import NotesDrawer from '@/bloom/notes/components/Drawer.vue';
import GalleryDrawer from '@/bloom/gallery/components/Drawer.vue';
import AdminDrawer from '@/bloom/admin/components/Drawer.vue';
import HelpDrawer from '@/bloom/help/components/Drawer.vue';


@Component({
  components: {
    'blm-admin-drawer': AdminDrawer,
    'blm-bitflow-drawer': BitflowDrawer,
    'blm-drive-drawer': DriveDrawer,
    'blm-gallery-drawer': GalleryDrawer,
    'blm-help-drawer': HelpDrawer,
    'blm-music-drawer': MusicDrawer,
    'blm-myaccount-drawer': MyAccountDrawer,
    'blm-notes-drawer': NotesDrawer,
    'blm-platform-drawer': PlatformDrawer,
  },
})
export default class Authenticated extends Vue {
  // props
  // data
  drawer = false;
  select_project_dialog = false;


  // computed
  // lifecycle
  created() {
    this.setup(false);
    // if (this.$vuetify.breakpoint.smAndDown) {
    //   this.drawer = false;
    // } else {
    //   this.drawer = true;
    // }
  }


  // watch
  @Watch('$route')
  on_route_change(_: Route, prev: Route) {
    let wasOpen = false;
    if (typeof prev.meta.service === 'string') {
      wasOpen = true;
    }

    this.setup(wasOpen);
  }


  // methods
  goto(path: string) {
    router.push({ path });
  }

  // TODO: close drawer on homepage resize
  setup(wasOpen: boolean) {
    if (typeof this.$route.meta.service !== 'string' && this.$route.meta.service !== '') {
      this.drawer = false; // close for homepage
    }
    if (this.$vuetify.breakpoint.mdAndDown || wasOpen) {
      return; // do not reopen on every route change within each service route
    }

    switch (this.$route.meta.service) {
      case 'myaccount':
        this.drawer = true;
        break;
      case 'drive':
        this.drawer = true;
        break;
      case 'bitflow':
        this.drawer = true;
        break;
      case 'platform':
        this.drawer = true;
        break;
      case 'notes':
        this.drawer = true;
        break;
      case 'contacts':
        this.drawer = false;
        break;
      case 'arcade':
        this.drawer = false;
        break;
      case 'gallery':
        this.drawer = true;
        break;
      case 'music':
        this.drawer = true;
        break;
      case 'admin':
        this.drawer = true;
        break;
      case 'help':
        this.drawer = true;
        break;
      default:
        this.drawer = false;
        break;
    }
  }

  sign_out() {
    api.sign_out();
  }
}
</script>


<style scoped lang="scss">
.v-toolbar a {
  color: white;
  text-decoration: none;
}

#menu-account {
  margin-left: calc(50% - 44px);
}

#layout-wrapper {
  height: 100%;
}

.v-content {
  height: 100%;
}

.lightbox {
  box-shadow: 0 0 40px inset rgba(0, 0, 0, 0.2);
  background-image: linear-gradient(to top, rgba(0, 0, 0, 0.4) 0%, transparent 72px);
}

.drawer-sign-out {
  position: absolute;
  bottom: 0;
}
</style>
