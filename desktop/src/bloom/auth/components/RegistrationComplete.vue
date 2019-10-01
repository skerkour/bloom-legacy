<template>
  <v-container grid-list-xl text-xs-center>
    <v-layout row wrap text-xs-center>

      <v-flex xs12>
        <p>
          Your username is unique and cannot be changed. <br/>
          It must be composed of only alphanumeric characters
        </p>
      </v-flex>

      <v-flex xs12 sm6 offset-sm3>
        <v-text-field
          v-model="username"
          label="Your username"
          :rules="usernameRules"
          :disabled="isLoading"
          outlined
          prefix="@"
          @keyup="usernameLower"
          @keyup.enter.native="completeRegistration"
        />
      </v-flex>

      <v-flex xs12 text-xs-center v-if="error">
        <v-alert icon="mdi-alert-circle" :value="error" type="error">
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
import { Native } from '@/native';


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

  // computed
  // lifecycle
  created() {
    // verify that we have the correct values in store
    if (this.$store.state.pending_account) {
      if (this.$store.state.pending_account.id) {
        this.pendingAccountId = this.$store.state.pending_account.id;
      }
    }

    if (!this.pendingAccountId) {
      this.$router.push({ path: '/auth/welcome' });
    }
  }

  // watch
  // methods
  async completeRegistration() {
    this.error = '';
    this.isLoading = true;
    const message = {
      type: 'auth.registration_complete',
      data: {
        id: this.pendingAccountId,
        username: this.username,
      },
    };
    try {
      const res = await Native.call(message);
      // api.sign_in(res);
      // router.push({ path: '/' });
      console.log('success');
      console.log(res);
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
