<template>
  <v-container fluid>
    <v-switch
      :input-value="$store.state.darkMode"
      :label="`${$store.state.darkMode ? 'Dark' : 'Light'}`"
      v-on:change="onDarkModeChanged"
    />
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Mutations } from '@/store';

@Component
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
  onDarkModeChanged() {
    // TODO(z0mbie42): save to settings
    this.$store.commit(Mutations.SWITCH_DARK_MODE.toString());
    this.$vuetify.theme.dark = this.$store.state.darkMode;
  }
}
</script>


<style lang="scss" scoped>
</style>
