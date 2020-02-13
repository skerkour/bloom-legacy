<template>
  <div>
    <v-row v-if="error !== ''">
      <v-col cols="12">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" sm="3">
        <v-subheader>Current password</v-subheader>
      </v-col>
      <v-col cols="12" sm="9" class="pb-0">
        <v-text-field
          placeholder="Current password"
          v-model="currentPassword"
          :type="showCurrentPassword ? 'text' : 'password'"
          :append-icon="showCurrentPassword ? 'mdi-eye-off' : 'mdi-eye'"
          @click:append="showCurrentPassword = !showCurrentPassword"
          :disabled="loading"
        />
        <router-link to="/myaccount">Forgot password?</router-link>
      </v-col>
    </v-row>


    <v-row>
      <v-col cols="12" sm="3">
        <v-subheader>New password</v-subheader>
      </v-col>
      <v-col cols="12" sm="9" class="pb-0">
        <v-text-field
          placeholder="New password"
          v-model="newPassword"
          :type="showNewPassword ? 'text' : 'password'"
          :append-icon="showNewPassword ? 'mdi-eye-off' : 'mdi-eye'"
          @click:append="showNewPassword = !showNewPassword"
          :disabled="loading"
        />
      </v-col>
    </v-row>


    <v-row>
      <v-col cols="12" sm="3">
        <v-subheader>Verify new password</v-subheader>
      </v-col>
      <v-col cols="12" sm="9" class="pb-0">
        <v-text-field
          placeholder="Verify new password"
          v-model="newPasswordVerification"
          :type="showPasswordVerification ? 'text' : 'password'"
          :append-icon="showPasswordVerification ? 'mdi-eye-off' : 'mdi-eye'"
          @click:append="showPasswordVerification = !showPasswordVerification"
          :disabled="loading"
        />
      </v-col>
    </v-row>


    <v-row>
      <v-col cols="12" class="text-right pt-0">
        <v-btn text @click="cancel">Cancel</v-btn>
        <v-btn color="primary" :disabled="!updatable" class="mx-3">Update</v-btn>
      </v-col>
    </v-row>

  </div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class ChangePasswordForm extends Vue {
  // props

  // data
  loading = false;
  currentPassword = '';
  showCurrentPassword = false;
  newPassword = '';
  showNewPassword = false;
  newPasswordVerification = '';
  showPasswordVerification = false;
  error = '';

  // computed
  get updatable() {
    return this.currentPassword !== ''
      && this.newPassword !== '' && this.newPasswordVerification !== '';
  }

  // lifecycle
  created() {
    this.cancel();
  }

  // watch
  // methods
  cancel() {
    this.currentPassword = '';
    this.showCurrentPassword = false;
    this.newPassword = '';
    this.newPasswordVerification = '';
    this.showPasswordVerification = false;
    this.error = '';
  }
}
</script>


<style lang="scss" scoped>
</style>
