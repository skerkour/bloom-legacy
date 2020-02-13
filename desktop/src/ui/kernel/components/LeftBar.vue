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
        >
          <template v-slot:activator="{ on }">
            <v-list-item-avatar
              v-on="on"
              class="blm-pointer"
            >
              <v-img :src="avatarUrl" />
            </v-list-item-avatar>

            <!-- <v-list-item-title>John Leider</v-list-item-title> -->
          </template>

          <v-card>
            <v-list>
              <v-list-item>
                <v-list-item-avatar>
                  <v-img :src="avatarUrl" alt="Profile Picture" />
                </v-list-item-avatar>

                <v-list-item-content class="text-left">
                  <v-list-item-title>{{ $store.state.me.displayName }}</v-list-item-title>
                  <v-list-item-subtitle>@{{ $store.state.me.username }}</v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>

              <v-list-item class="justify-center">
                <v-btn
                  text
                  to="/myaccount"
                  @click="accountMenu = false"
                >
                  <v-icon left>mdi-account</v-icon> My Account
                </v-btn>

                <v-btn
                  text
                  to="/preferences"
                  @click="accountMenu = false"
                >
                  <v-icon left>mdi-settings</v-icon> Preferences
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
import getApps from '@/ui/kernel/apps';

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
  apps = getApps();

  // computed
  get avatarUrl() {
    return this.$store.state.me?.avatarUrl ? this.$store.state.me?.avatarUrl : '/assets/imgs/profile.jpg';
  }

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
  overflow-y: auto;
  width: 72px !important;
}
</style>
