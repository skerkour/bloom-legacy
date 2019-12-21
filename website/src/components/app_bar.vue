<template>
  <v-app-bar app elevation="0">
    <router-link to="/" class="blm-appbar-title">
      <v-toolbar-title class="headline" to="/">
        <span class="blm-rounded-elegance"><b>Bloom</b></span>
      </v-toolbar-title>
    </router-link>

    <v-spacer></v-spacer>

    <v-toolbar-items>
      <v-btn
        text
        to="/download"
      >{{ $t('appbar.download') }}</v-btn>
      <v-btn
        text
        to="/features"
        exact
      >{{ $t('appbar.features') }}</v-btn>
      <!-- <v-btn text>Help</v-btn> -->
      <blm-locales-menu />
    </v-toolbar-items>
  </v-app-bar>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import LocalesMenu from '@/components/locales_menu.vue';

@Component({
  components: {
    'blm-locales-menu': LocalesMenu,
  },
})
export default class AppBar extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style lang="scss" scoped>
.blm-appbar-title {
  color: inherit;
}
</style>
