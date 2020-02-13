<template>
<div>
  <v-content>
    <v-container grid-list-xl text-xs-center fluid>
      <v-flex xs12 sm8 md6 offset-sm2 offset-md3 id="main">
        <div id="main-card">

          <v-tabs fixed-tabs class="mb-5" background-color="transparent" v-if="showSurroundings">
            <v-tab ripple to="/auth/registration/start">Create account</v-tab>
            <v-tab ripple to="/auth/sign-in">Sign in</v-tab>
          </v-tabs>
          <router-view></router-view>

        <div class="mb-5 mt-5" v-if="showSurroundings">
          <router-link to="/myaccount/recovery/request">Forgot account?</router-link>
        </div>
        <!-- <div>
          <a @click="openHelp">Help</a>
        </div> -->

        </div>
      </v-flex>
    </v-container>

    <v-footer fixed color="transparent">
      <div class="flex-grow-1"></div>

      <v-tooltip top>
        <template v-slot:activator="{ on }">
          <v-btn text icon @click="openWebsite" v-on="on">
            <v-icon>mdi-information-outline</v-icon>
          </v-btn>
        </template>
        <span>Information</span>
      </v-tooltip>

      <v-tooltip top>
        <template v-slot:activator="{ on }">
          <v-btn icon @click="openHelp" v-on="on">
            <v-icon>mdi-help-circle-outline</v-icon>
          </v-btn>
        </template>
        <span>Help</span>
      </v-tooltip>

      <!-- <v-tooltip top>
        <template v-slot:activator="{ on }">
          <v-btn icon @click="openSettingsDialog" v-on="on">
            <v-icon>mdi-settings</v-icon>
          </v-btn>
        </template>
        <span>Settings</span>
      </v-tooltip> -->

      <!-- <div>&copy; {{ new Date().getFullYear() }}</div> -->
    </v-footer>
  </v-content>

  <blm-dialog-settings
    :visible="showSettingsDialog"
    @closed="closeSettingsDialog"
  />
</div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { shell } from 'electron';
import SettingsDialog from '@/ui/auth/components/SettingsDialog.vue';
import config from '@/config';

@Component({
  components: {
    'blm-dialog-settings': SettingsDialog,
  },
})
export default class Auth extends Vue {
  // props
  // data
  showSettingsDialog = false;

  // computed
  get showSurroundings(): boolean {
    const route = this.$route.path;
    if (route === '/auth/registration/start' || route === '/auth/sign-in') {
      return true;
    }

    return false;
  }
  // lifecycle
  // watch
  // methods
  openHelp() {
    console.log('opening help');
    console.log(shell);
    console.log(config);
    shell.openExternal(config.HELP_URL);
  }

  openWebsite() {
    shell.openExternal(config.WEBSITE_URL);
  }

  openSettingsDialog() {
    this.showSettingsDialog = true;
  }

  closeSettingsDialog() {
    this.showSettingsDialog = false;
  }
}
</script>


<style lang="scss" scoped>
@media screen and (min-width: 600px) {
  #main-card {
    margin-top: 50px;
  }
}

.v-tabs__div {
  text-transform: none; /* overwrite default 'uppercase' */
}

#main {
  margin-bottom: 20px;
}

// .tab-item {
//   padding: 20px;
// }

#help {
  position: absolute;
  bottom: 2px;
  left: calc(50vw - 15px);
  color: #aaa;
  text-decoration: none;
}
</style>
