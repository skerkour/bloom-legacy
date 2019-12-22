<template>
  <p class="subtitle-1 mt-5">
    <router-link to="/download" class="text--darken-1 grey--text">
      {{ $t('labels.other_downloads') }}
    </router-link>
  </p>
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
</style>
