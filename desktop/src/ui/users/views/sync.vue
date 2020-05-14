<template>
  <v-container fluid>
    <v-row align="center" justify="center">
        <v-progress-circular
        indeterminate
        color="primary"
      ></v-progress-circular>
      <h1 class="headline">Syncing data</h1>
      <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
        {{ error }}
      </v-alert>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import core from '@/core';
import { Method } from '@/core/users';

@Component
export default class Synv extends Vue {
  // props
  // data
  error = '';
  // computed
  // lifecycle
  created() {
    this.sync();
  }
  // watch
  // methods
  async sync() {
    this.error = '';
    try {
      await core.call(Method.Sync, core.Empty);
      this.$router.push({ path: '/' });
    } catch (err) {
      this.error = err.message;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
