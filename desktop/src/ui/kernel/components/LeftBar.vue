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
        <v-list-item-avatar color="white" class="blm-pointer" @click="goto('/myaccount')">
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-img :src="$store.state.me.avatarUrl" v-if="$store.state.me.avatarUrl" v-on="on"/>
              <v-icon medium color="grey" v-else v-on="on">mdi-account</v-icon>
            </template>
            <span>My Account</span>
          </v-tooltip>
        </v-list-item-avatar>
      </v-list-item>

      <v-list-item>
        <v-list-item-avatar color="white" class="blm-pointer" @click="goto('/preferences')">
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
               <v-icon medium color="grey" v-on="on">mdi-cog</v-icon>
            </template>
            <span>Preferences</span>
          </v-tooltip>
        </v-list-item-avatar>
      </v-list-item>

      <v-list-item>
        <v-list-item-avatar color="white" class="blm-pointer" @click="goto('/groups')">
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
               <v-icon medium color="grey" v-on="on">mdi-account-group</v-icon>
            </template>
            <span>Groups</span>
          </v-tooltip>
        </v-list-item-avatar>
      </v-list-item>

      <v-list-item>
        <v-list-item-avatar color="white" class="blm-pointer" @click="openHelp">
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
               <v-icon medium color="grey" v-on="on">mdi-help</v-icon>
            </template>
            <span>Help</span>
          </v-tooltip>
        </v-list-item-avatar>
      </v-list-item>

      <v-list-item>
        <v-list-item-avatar color="white" class="blm-pointer" @click="openAllAppsDialog">
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
               <v-icon medium color="grey" v-on="on">mdi-apps</v-icon>
            </template>
            <span>All apps</span>
          </v-tooltip>
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
import config from '@/config';

const { shell } = window as any;

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

  openHelp() {
    shell.openExternal(config.HELP_URL);
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
