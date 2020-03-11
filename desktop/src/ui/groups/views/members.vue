<template>
  <v-container fluid class="text-left">
    <v-row>
      <v-alert type="error" :value="error !== ''">
        {{ error }}
      </v-alert>
    </v-row>

    <v-row>
      <v-toolbar flat dense>
        <v-btn color="primary" @click="openInviteDialog">
          <v-icon left>mdi-plus</v-icon> Invite
        </v-btn>
      </v-toolbar>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Groups extends Vue {
  // props
  // data
  error = '';
  showInviteDialog = false;

  // computed
  // lifecycle
  // watch
  // methods
  openInviteDialog() {
    this.showInviteDialog = true;
  }
}
</script>


<style lang="scss" scoped>
</style>
