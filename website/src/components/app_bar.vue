<template>
  <v-app-bar
    app
    elevation="0"
    color="transparent"
    elevate-on-scroll
  >
    <router-link to="/">
      <v-toolbar-title
        class="headline"
        to="/"
      >
        <span class="blm-toolbar-title-primary blm-rounded-elegance"><b>Bloom</b></span>
      </v-toolbar-title>
    </router-link>

    <v-spacer></v-spacer>

    <v-toolbar-items>
      <v-btn
        text
        to="/download"
      >Download</v-btn>
      <v-btn
        text
        to="/features"
      >Features</v-btn>
      <v-btn text>Help</v-btn>
    </v-toolbar-items>
  </v-app-bar>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class AppBar extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style>
a {
  text-decoration: none;
  color: none;
}

.blm-toolbar-title-primary {
  color: black;
}
</style>
