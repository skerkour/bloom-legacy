<template>
  <v-container class="fill-height" fluid>
    <v-row align="center" justify="center">
      <v-btn color="error" depressed @click="openConfirmDialog">
        <v-icon left>mdi-power</v-icon> Sign Out
      </v-btn>
      <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
        {{ error }}
      </v-alert>
    </v-row>


    <v-dialog v-model="confirmDialog" width="500">
      <v-card class="text-left">
        <v-card-title class="headline" primary-title>
          Confirm
        </v-card-title>

        <v-card-text>
          Do you really want to sign out?
        </v-card-text>

        <v-card-actions>
          <v-spacer />
          <v-btn text @click="cancel">
            Cancel
          </v-btn>
          <v-btn color="error" @click="signOut">
            <v-icon left>mdi-power</v-icon> Yes! Sign me out
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import core from '@/core';
import { Method } from '@/core/users';
import { Mutations } from '@/store';

@Component
export default class BlmSignOut extends Vue {
  // props
  // data
  confirmDialog = false;
  loading = false;
  error = '';
  // computed
  // lifecycle
  // watch
  // methods


  async signOut() {
    this.closeConfirmDialog();
    this.error = '';
    this.loading = true;
    try {
      await core.call(Method.SignOut, core.Empty);
      this.$store.commit(Mutations.SIGN_OUT.toString());
      window.location.reload();
    } catch (err) {
      this.error = err.message;
    }
  }

  cancel() {
    this.closeConfirmDialog();
  }

  openConfirmDialog() {
    this.confirmDialog = true;
  }

  closeConfirmDialog() {
    this.confirmDialog = false;
  }
}
</script>


<style lang="scss" scoped>
</style>
