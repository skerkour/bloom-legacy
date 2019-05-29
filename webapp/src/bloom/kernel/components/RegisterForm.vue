<template>
  <v-container grid-list-xl text-xs-center>
    <v-layout row wrap text-xs-center>

      <v-flex xs12 sm4 offset-sm2>
        <v-text-field
          label="First name"
          type="text"
          v-model="first_name"
          :rules="first_name_rules"
          :disabled="is_loading"
          ></v-text-field>
      </v-flex>
      <v-flex xs12 sm4>
        <v-text-field
          label="Last name"
          type="text"
          v-model="last_name"
          :rules="last_name_rules"
          :disabled="is_loading"
          ></v-text-field>
      </v-flex>

      <v-flex xs12 sm8 offset-sm2>
        <v-text-field
          label="Email"
          type="email"
          v-model="email"
          :rules="email_rules"
          :disabled="is_loading"
          @keyup="lowercase_email"
          ></v-text-field>
      </v-flex>

      <v-flex xs12 sm8 offset-sm2>
        <v-text-field
          label="Password"
          v-model="password"
          :rules="password_rules"
          :type="show_password ? 'text' : 'password'"
          :append-icon="show_password ? 'mdi-eye-off' : 'mdi-eye'"
          @click:append="show_password = !show_password"
          :disabled="is_loading"
          @keyup.enter.native="register"
          ></v-text-field>
      </v-flex>

      <v-flex xs12 text-xs-center v-if="error">
        <v-alert icon="mdi-alert-circle" :value="error.length !== 0" type="error">
          {{ error }}
        </v-alert>
      </v-flex>

      <v-flex xs12 sm8 offset-sm2 md6 offset-md3 class="text-xs-left disclaimer mb-4">
        By creating an account, I agree to the
        <a href="/terms" target="_blank" rel="noopener">Terms of Service</a>
        and <a href="/privacy" target="_blank" rel="noopener">Privacy policy</a>.
      </v-flex>

      <v-flex xs12 text-xs-center id="flex-btn">
        <v-btn color="primary"  @click="register" :loading="is_loading">
          Create Account
        </v-btn>
      </v-flex>

    </v-layout>
  </v-container>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import router from '@/bloom/kernel/router';


@Component
export default class RegisterForm extends Vue {
  show_password = false;
  first_name = '';
  first_name_rules = [
    (v: string) => !!v || 'First name is required',
  ];
  last_name = '';
  last_name_rules = [
    (v: string) => !!v || 'Last name is required',
  ];
  email = '';
  email_rules = [
    (v: string) => !!v || 'Email name is required',
    (v: string) => v.indexOf('@') !== -1 || 'Email is not valid',
  ];
  password = '';
  password_rules = [
    (v: string) => !!v || 'Password is required',
    (v: string) => (v && v.length > 7) || 'Password must be at leat 8 characters',
  ];
  is_loading = false;
  is_valid = false;
  error = '';


  async register() {
    this.is_loading = true;
    this.error = '';
    const payload = {
      email: this.email,
      first_name: this.first_name,
      last_name: this.last_name,
      password: this.password,
    };
    try {
      const res = await api.post(`${api.MYACCOUNT}/v1/registration/start`, payload);

      const pending_account = {
        email: this.email,
        id: res.id,
      };
      this.$store.commit('set_pending_account', pending_account);
      router.push({ path: '/welcome/verify', query: this.$route.query });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  lowercase_email() {
    this.email = this.email.toLowerCase();
  }

}
</script>


<style scoped lang="scss">
#flex-btn {
  margin-top: -28px;
}

.disclaimer {
  a {
    text-decoration: none;
  }
}
</style>
