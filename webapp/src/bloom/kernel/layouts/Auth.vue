<template>
  <v-container grid-list-xl text-xs-center>
    <v-flex xs12 sm8 md6 offset-sm2 offset-md3 id="main">
      <div id="main-card" class="elevation-0">

        <v-tabs fixed-tabs>
          <v-tab ripple to="/sign-in">Sign in</v-tab>
          <v-tab ripple to="/register">Create account</v-tab>
        </v-tabs>
        <router-view></router-view>

      <div id="forgot-link">
        <router-link to="/myaccount/recovery/request">Forgot account?</router-link>
      </div>
      <div id="help-link">
        <a href="https://help.bloom.sh" target="_blank" rel="noopener">Help</a>
      </div>

      </div>
    </v-flex>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component
export default class Auth extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">

@media screen and (min-width: 600px) {
  #main-card {
    margin-top: 50px;
  }
}

.v-tabs__div {
  text-transform: none; /* overwrite default 'uppercase' */
}

#main-card {
  border-radius: 4px;
}

a {
  text-decoration: none;
}
a:hover {
  text-decoration: underline;
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

#forgot-link {
  margin-top: 8px;
  margin-bottom: 30px;
}
</style>
