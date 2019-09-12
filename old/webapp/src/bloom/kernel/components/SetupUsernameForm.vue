<template>
  <v-layout row wrap text-xs-center>
    <v-flex xs12 sm8 md6 offset-sm2 offset-md3 id="main" class="text-xs-center">
      <div class="elevation-0">
        <v-container fluid fill-height>
          <v-layout row wrap text-xs-center>
            <v-flex xs12 class="mb-2">
              <p>
                Your username is unique and cannot be changed. <br/>
                It must be composed of only alphanumeric characters
              </p>
            </v-flex>
            <v-flex xs12 sm8 offset-sm2>
              <v-text-field
              v-model="username"
              label="Your username"
              :disabled="is_loading"
              outline
              :rules="username_rules"
              prefix="@"
              @keyup.enter.native="complete_registration"
              @keyup="username_fn"
              ></v-text-field>
            </v-flex>
            <v-flex xs12 text-xs-center>
              <v-alert icon="mdi-alert-circle" :value="error" type="error">
                {{ error }}
              </v-alert>
            </v-flex>
            <v-flex xs12>
              <v-btn color="primary" @click="complete_registration" :loading="is_loading">
                Complete registration
              </v-btn>
            </v-flex>
          </v-layout>
        </v-container>
      </div>
    </v-flex>
  </v-layout>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import router from '@/bloom/kernel/router';
import api from '@/bloom/kernel/api';


@Component
export default class SetupUsernameForm extends Vue {
  error = '';
  is_loading = false;
  code = '';
  pending_account_id = '';
  username = '';
  username_rules = [
    (v: string) => !!v || 'Username is required',
    (v: string) => (v && v.length > 3) || 'Username must be at least 4 characters',
  ];


  created() {
    // verify that we have the correct values in store
    if (this.$store.state.pending_account) {
      if (this.$store.state.pending_account.id) {
        this.pending_account_id = this.$store.state.pending_account.id;
      }
    }

    if (!this.pending_account_id) {
      router.push({ path: '/register' });
    }
  }

  async complete_registration() {
    this.error = '';

    const verification = {
      id: this.pending_account_id,
      username: this.username,
    };
    this.is_loading = true;
    try {
      const res = await api.post(`${api.MYACCOUNT}/v1/registration/complete`, verification);
      api.sign_in(res);
      router.push({ path: '/' });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  username_fn() {
    this.username = this.username.toLowerCase();
  }
}
</script>
