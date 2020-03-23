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
import core from '@/core';
import { Method, SetParams } from '@/core/preferences';

@Component
export default class Index extends Vue {
  // props
  // data
  error = '';
  loading = false;

  // computed
  // lifecycle
  // watch
  // methods
  async onDarkModeChanged() {
    this.error = '';
    this.loading = true;
    const params: SetParams = {
      key: 'theme',
      value: this.$store.state.darkMode ? 'light' : 'dark',
    };

    try {
      await core.call(Method.Set, params);
      this.$store.commit(Mutations.SWITCH_DARK_MODE.toString());
      this.$vuetify.theme.dark = this.$store.state.darkMode;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
