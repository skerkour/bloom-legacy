<template>
  <v-container >
    <v-layout row wrap>

      <v-flex xs12>
        <h1 class="display-1 font-weight-medium">
          MyAccount
        </h1>
        <p class="font-18 mt-2">
          The <router-link to="/myaccount">MyAccount app</router-link> allows you to manage your bloom account,
          whether it be to update your profile or change your password.
        </p>
      </v-flex>


      <v-flex xs12 class="mt-4">
        <h2 class="headline font-weight-medium">
          Dark theme
        </h2>
        <p class="font-18 mt-2">
          The switch to activate / deactivate the dark theme can be found at the bottom of your profile page, in
          the <router-link to="/myaccount">MyAccount app</router-link>.
        </p>
        <v-img max-height="600px" contain src="/help/static/myaccount/dark_mode.png" />
      </v-flex>

    </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
a {
  text-decoration: none;
}

.font-18 {
  font-size: 18px;
}
</style>
