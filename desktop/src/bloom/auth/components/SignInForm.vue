<template>
  <v-container grid-list-xl text-xs-center>

    <v-flex xs12 sm8 offset-sm2>
      <v-text-field
      type="text"
      label="Username"
      v-model="username"
      :disabled="isLoading"
      @keyup="lowercaseUsername"
      ></v-text-field>
    </v-flex>

    <v-flex xs12 sm8 offset-sm2>
      <v-text-field
      label="Password"
      v-model="password"
      :type="showPassword ? 'text' : 'password'"
      :append-icon="showPassword ? 'mdi-eye-off' : 'mdi-eye'"
      @click:append="showPassword = !showPassword"
      :disabled="isLoading"
      @keyup.enter.native="signIn"
      ></v-text-field>
    </v-flex>

    <v-flex xs12 text-xs-center>
      <v-alert icon="mdi-alert-circle" :value="error" type="error">
        {{ error }}
      </v-alert>
    </v-flex>

    <v-flex xs12 text-xs-center>
      <v-btn color="primary" @click="signIn" :loading="isLoading">
        Sign in
      </v-btn>
    </v-flex>

  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import router from '@/router'; // eslint-disable-line import/no-cycle

@Component
export default class SignInForm extends Vue {
  // props
  // data
  showPassword = false;
  isLoading = false;
  username = '';
  password = '';
  error: string | null = null;


  // computed
  // lifecycle
  // watch
  // methods
  async signIn() {
    console.log('signIn clicked');
  }

  lowercaseUsername() {
    this.username = this.username.toLowerCase();
  }
}
</script>

<style lang="scss" scoped>
</style>
