<template>
  <v-container grid-list-xl text-xs-center>
    <v-layout row wrap text-xs-center>

      <v-flex xs12>
        <p>
          Your username is unique and cannot be changed. <br/>
          It must be composed of only alphanumeric characters
        </p>
      </v-flex>

      <v-flex xs12 sm8 offset-sm2>
        <v-text-field
          v-model="username"
          label="Your username"
          :rules="usernameRules"
          :disabled="isLoading"
          prefix="@"
          @keyup="usernameLower"
          @keyup.enter.native="completeRegistration"
        />
      </v-flex>

      <v-flex xs12 sm8 offset-sm2>
        <v-text-field
          label="Password"
          v-model="password"
          :rules="passwordRules"
          :type="showPassword ? 'text' : 'password'"
          :append-icon="showPassword ? 'mdi-eye-off' : 'mdi-eye'"
          @click:append="showPassword = !showPassword"
          :disabled="isLoading"
          @keyup.enter.native="register"
        />
      </v-flex>

      <v-flex xs12 text-xs-center v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-flex>

      <v-flex xs12>
        <v-btn color="primary" @click="completeRegistration" :loading="isLoading">
          Complete registration
        </v-btn>
      </v-flex>

    </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Native, Message } from '@/native';
import { Mutations } from '@/store';

const { log } = require('@bloom42/astro');

@Component
export default class RegistrationComplete extends Vue {
  // props
  // data
  error = '';
  isLoading = false;
  code = '';
  pendingAccountId = '';
  username = '';
  usernameRules = [
    (v: string) => !!v || 'Username is required',
    (v: string) => (v && v.length > 3) || 'Username must be at least 4 characters',
  ];
  password = '';
  passwordRules = [
    (v: string) => !!v || 'Password is required',
    (v: string) => (v && v.length > 7) || 'Password must be at least 8 characters',
  ];
  showPassword = false;

  // computed
  // lifecycle
  created() {
    // collect value from route params
    if (this.$store.state.pending_account) {
      this.pendingAccountId = this.$store.state.pending_account.id;
    } else {
      this.$router.push({ path: '/auth/registration/start' });
    }
  }

  // watch
  // methods
  async completeRegistration() {
    this.error = '';
    this.isLoading = true;
    const message: Message = {
      type: 'auth.gui.registration_complete',
      data: {
        id: this.pendingAccountId,
        username: this.username,
        password: this.password,
      },
    };
    try {
      const res = await Native.call(message);
      log.debug(res);
      this.$store.commit(Mutations.CLEAR_PENDING_ACCOUNT.toString());
      this.$store.commit(Mutations.SIGN_IN.toString());
      this.$router.push({ path: '/' });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  usernameLower() {
    this.username = this.username.toLowerCase();
  }
}
</script>


<style lang="scss" scoped>
</style>
