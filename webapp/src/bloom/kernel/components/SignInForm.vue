<template>
  <v-container grid-list-xl text-xs-center>

    <v-flex xs12 sm8 offset-sm2>
      <v-text-field
      type="text"
      label="Username"
      v-model="username"
      :disabled="is_loading"
      @keyup="lowercase_username"
      ></v-text-field>
    </v-flex>

    <v-flex xs12 sm8 offset-sm2>
      <v-text-field
      label="Password"
      v-model="password"
      :type="show_password ? 'text' : 'password'"
      :append-icon="show_password ? 'mdi-eye-off' : 'mdi-eye'"
      @click:append="show_password = !show_password"
      :disabled="is_loading"
      @keyup.enter.native="sign_in"
      ></v-text-field>
    </v-flex>

    <v-flex xs12 text-xs-center>
      <v-alert icon="mdi-alert-circle" :value="error" type="error">
        {{ error }}
      </v-alert>
    </v-flex>

    <v-flex xs12 text-xs-center>
      <v-btn color="primary" @click="sign_in" :loading="is_loading">
        Sign in
      </v-btn>
    </v-flex>

  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import router from '@/bloom/kernel/router';

@Component
export default class SignInForm extends Vue {
  // props
  // data
  show_password = false;
  is_loading = false;
  username = '';
  password = '';
  error = '';


  // computed
  // lifecycle
  // watch
  // methods
  async sign_in() {
    this.error = '';
    const sign_in = {
      password: this.password,
      username: this.username,
    };
    try {
      this.is_loading = true;
      const res = await api.post(`${api.MYACCOUNT}/v1/sign-in`, sign_in);
      api.sign_in(res);
      router.push({ path: '/' });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  lowercase_username() {
    this.username = this.username.toLowerCase();
  }
}
</script>

