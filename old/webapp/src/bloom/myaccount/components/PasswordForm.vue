<template>
  <div class="elevation-0">
    <v-card-title>
      <div class="headline text-xs-left">
        Change password
      </div>
      <div class="text-xs-left">
        All other devices will be disconnected
      </div>
    </v-card-title>
      <v-container grid-list-xl text-xs-left>
        <v-layout row wrap justify-left>
          <v-flex hidden-xs-only sm4>
            <v-subheader>Current password</v-subheader>
          </v-flex>
          <v-flex xs12 sm8>
            <v-text-field
            v-model="current_password"
            label="Current password"
            :type="show_current_password ? 'text' : 'password'"
            :append-icon="show_current_password ? 'mdi-eye-off' : 'mdi-eye'"
            @click:append="show_current_password = !show_current_password"
            :disabled="is_loading"
            ></v-text-field>
            <router-link to="/myaccount/recovery/request">Forgot password?</router-link>
          </v-flex>

          <v-flex hidden-xs-only sm4>
            <v-subheader>New password</v-subheader>
          </v-flex>
          <v-flex xs12 sm8>
            <v-text-field
            v-model="new_password"
            label="New password"
            :type="show_password ? 'text' : 'password'"
            :append-icon="show_password ? 'mdi-eye-off' : 'mdi-eye'"
            @click:append="show_password = !show_password"
            :disabled="is_loading"
            ></v-text-field>
          </v-flex>

          <v-flex hidden-xs-only sm4>
            <v-subheader>Verify password</v-subheader>
          </v-flex>
          <v-flex xs12 sm8>
            <v-text-field
            v-model="new_password_verification"
            label="Verify password"
            :type="show_password_verification ? 'text' : 'password'"
            :append-icon="show_password_verification ? 'mdi-eye-off' : 'mdi-eye'"
            @click:append="show_password_verification = !show_password_verification"
            :disabled="is_loading"
            @keyup.enter.native="update_password"
            ></v-text-field>
          </v-flex>

          <v-flex xs12 text-xs-center v-if="error">
            <v-alert icon="mdi-alert-circle" :value="error" type="error">
              {{ error }}
            </v-alert>
          </v-flex>

          <v-flex class="text-xs-right" xs12>
            <v-btn color="primary" :loading="is_loading" @click="update_password"
            :disabled="!current_password && !new_password && !new_password_verification"
            >
            Update
          </v-btn>
        </v-flex>

        </v-layout>
      </v-container>
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class PasswordForm extends Vue {
  // props
  // data
  is_loading = false;
  current_password = '';
  show_current_password = false;
  new_password = '';
  show_password = false;
  new_password_verification = '';
  show_password_verification = false;
  error = '';


  // computed
  // lifecycle
  // watch
  // methods
  async update_password() {
    this.error = '';
    this.show_password = false;
    this.show_current_password = false;
    this.show_password_verification = false;
    if (this.new_password_verification !== this.new_password) {
      this.error = 'new password verification does not match';
      return;
    }
    try {
      this.is_loading = true;
      const payload = {
        current_password: this.current_password,
        new_password: this.new_password,
      };
      const res = await api.put(`${api.MYACCOUNT}/v1/me/password`, payload);
      this.$toast.success('Success', { icon: 'mdi-check-circle' });
      this.current_password = '';
      this.new_password = '';
      this.new_password_verification = '';
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
.headline {
  width: 100%;
}

a {
  text-decoration: none;
}
a:hover {
  text-decoration: underline;
}
</style>
