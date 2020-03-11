<template>
  <v-container fluid class="text-left">
    <v-row>
      <v-col cols="12" v-if="error">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
      <v-col cols="12" v-if="loading">
        <v-progress-circular
          indeterminate
          color="primary"
          size="50"
        />
      </v-col>
    </v-row>

    <v-row>
      <v-toolbar flat dense>
        <v-btn color="primary" @click="openInviteDialog">
          <v-icon left>mdi-plus</v-icon> Invite
        </v-btn>
      </v-toolbar>
    </v-row>

    <blm-groups-invite-dialog :visible="showInviteDialog"
      @closed="inviteDialogClosed" />
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmGroupsInviteDialog from '../components/invite_dialog.vue';

@Component({
  components: {
    BlmGroupsInviteDialog,
  },
})
export default class MembersView extends Vue {
  // props
  // data
  loading = false;
  error = '';
  showInviteDialog = false;

  // computed
  // lifecycle
  // watch
  // methods
  openInviteDialog() {
    this.showInviteDialog = true;
  }

  inviteDialogClosed() {
    this.showInviteDialog = false;
  }
}
</script>


<style lang="scss" scoped>
</style>
