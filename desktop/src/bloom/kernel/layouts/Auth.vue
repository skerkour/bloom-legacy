<template>
  <v-content>
    <v-container grid-list-xl text-xs-center>
      <v-flex xs12 sm8 md6 offset-sm2 offset-md3 id="main">
        <div id="main-card">

          <v-tabs fixed-tabs class="mb-5">
            <v-tab ripple to="/sign-in">Sign in</v-tab>
            <v-tab ripple to="/register">Create account</v-tab>
          </v-tabs>
          <router-view></router-view>

        <div class="mb-5 mt-5">
          <router-link to="/myaccount/recovery/request">Forgot account?</router-link>
        </div>
        <div>
          <a @click="openHelp">Help</a>
        </div>

        </div>
      </v-flex>
    </v-container>
  </v-content>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

const { shell } = (window as any).require('electron');
const config = require('@/config');


@Component
export default class Auth extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
  openHelp() {
    shell.openExternal(config.HELP_URL);
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

.tab-item {
  padding: 20px;
}

#help {
  position: absolute;
  bottom: 2px;
  left: calc(50vw - 15px);
  color: #aaa;
  text-decoration: none;
}

.theme--light.v-tabs>.v-tabs-bar {
  background-color: transparent;
}
</style>
