<template>
  <v-container class="fill-height" fluid>
    <v-row align="center" justify="center">
      <v-btn color="error" depressed @click="openConfirmDialog">
        <v-icon left>mdi-power</v-icon> Sign Out
      </v-btn>
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
          <v-btn color="error" text @click="signOut">
            <v-icon left>mdi-power</v-icon> Yes! Sign me out
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class BlmSignOut extends Vue {
  // props
  // data
  confirmDialog = false;
  // computed
  // lifecycle
  // watch
  // methods


  signOut() {
    this.closeConfirmDialog();
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
