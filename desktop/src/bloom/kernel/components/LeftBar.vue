<template>
<div>
  <v-navigation-drawer
    mini-variant
    permanent
    absolute
    fill-height
    dark
    class="blm-left-bar"
  >
    <v-list>
      <v-list-item>
        <v-menu
          v-model="accountMenu"
          :close-on-content-click="false"
          :nudge-width="200"
          offset-x
        >
          <template v-slot:activator="{ on }">
            <v-list-item-avatar
              v-on="on"
              class="blm-pointer"
            >
              <v-img src="https://randomuser.me/api/portraits/men/85.jpg"></v-img>
            </v-list-item-avatar>

            <!-- <v-list-item-title>John Leider</v-list-item-title> -->
          </template>

          <v-card>
            <v-list>
              <v-list-item>
                <v-list-item-avatar>
                  <v-img src="https://cdn.vuetifyjs.com/images/john.jpg" alt="Profile Picture" />
                </v-list-item-avatar>

                <v-list-item-content>
                  <v-list-item-title>Sylvain Kerkour</v-list-item-title>
                  <v-list-item-subtitle>z0mbie42</v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>

              <v-list-item class="justify-center">
                <v-btn
                  text
                  color="primary"
                  to="/preferences"
                  id="menu-account"
                  class="elevation-0"
                  @click="accountMenu = false"
                >
                  Preferences
                </v-btn>
              </v-list-item>
            </v-list>

          </v-card>
        </v-menu>
      </v-list-item>

      <v-list-item>
        <v-list-item-avatar
          color="white"
          class="blm-pointer"
        >
          <v-icon
            medium
            color="grey"
          >mdi-account-group</v-icon>
        </v-list-item-avatar>

        <!-- <v-list-item-title>John Leider</v-list-item-title> -->
      </v-list-item>

      <v-list-item>
        <v-list-item-avatar
          color="white"
          class="blm-pointer"
          @click="openAllAppsDialog"
        >
          <v-icon
            medium
            color="grey"
          >mdi-apps</v-icon>
        </v-list-item-avatar>
      </v-list-item>

      <v-list-item
        v-for="(app, index) in apps"
        :key="index"
      >
        <v-list-item-avatar
          class="blm-pointer"
          @click="goto(app.path)"
        >
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-img
                :src="app.icon"
                v-on="on"
              />
            </template>
            <span>{{ app.name }}</span>
          </v-tooltip>
        </v-list-item-avatar>
      </v-list-item>

    </v-list>
  </v-navigation-drawer>

  <blm-dialog-all-apps
    :visible="showAllAppsDialog"
    @closed="closeAllAppsDialog"
  />
</div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import AllAppsDialog from './AllAppsDialog.vue';


@Component({
  components: {
    'blm-dialog-all-apps': AllAppsDialog,
  },
})
export default class LeftBar extends Vue {
  // props
  // data
  accountMenu = false;
  showAllAppsDialog = false;
  apps = [
    // {
    //   name: 'Chat',
    //   path: '/chat',
    //   icon: '/kernel/imgs/icons/chat_512.jpg',
    // },
    // {
    //   name: 'Music',
    //   path: '/music',
    //   icon: '/kernel/imgs/icons/music_512.jpg',
    // },
    // {
    //   name: 'Gallery',
    //   path: '/gallery',
    //   icon: '/kernel/imgs/icons/gallery_512.jpg',
    // },
    {
      name: 'Notes',
      path: '/notes',
      icon: '/assets/icons/notes_256.jpg',
    },
    {
      name: 'Calendar',
      path: '/calendar',
      icon: '/assets/icons/calendar_256.jpg',
    },
    // {
    //   name: 'Bitflow',
    //   path: '/bitflow',
    //   icon: '/kernel/imgs/icons/bitflow_512.jpg',
    // },
    {
      name: 'Arcade',
      path: '/arcade',
      icon: '/assets/icons/arcade_256.jpg',
    },
    {
      name: 'Contacts',
      path: '/contacts',
      icon: '/assets/icons/contacts_256.jpg',
    },
    // {
    //   name: 'Drive',
    //   path: '/drive',
    //   icon: '/kernel/imgs/icons/drive_512.jpg',
    // },
    {
      name: 'Calculator',
      path: '/calculator',
      icon: '/assets/icons/calculator_256.jpg',
    },
  ];


  // computed
  // lifecycle
  // watch
  // methods
  openAllAppsDialog() {
    this.showAllAppsDialog = true;
  }

  closeAllAppsDialog() {
    this.showAllAppsDialog = false;
  }

  goto(path: string) {
    this.$router.push({ path }).catch(() => {});
  }
}
</script>


<style lang="scss" scoped>
.blm-left-bar {
  z-index: 100;
}
</style>
