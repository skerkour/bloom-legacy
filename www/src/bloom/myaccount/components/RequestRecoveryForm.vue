<template>
<v-container>
  <v-layout row wrap text-xs-center>
    <v-flex xs12 sm8 md6 offset-sm2 offset-md3 id="main" class="text-xs-center">
      <v-card class="elevation-0">
        <div>
          <h2>Account recovery</h2>
          <p>
            To recover your account and reset your password please enter your <b>email</b>
            or your <b>username</b> in the text field below.
            <br/>
            Please note that all your devices will be disconnected for security reasons.
          </p>
        </div>
        <v-layout row wrap text-xs-center>
          <v-flex xs12 sm8 offset-sm2>
            <v-text-field
            v-model="email_or_username"
            label="Email or Username"
            :disabled="is_loading"
            outline
            @keyup.enter.native="request_recovery"
            @keyup="lowercase_email_or_username"
            ></v-text-field>
          </v-flex>

          <v-flex xs12 text-xs-center>
            <v-alert :value="error" type="error">
              {{ error }}
            </v-alert>
          </v-flex>

          <v-flex xs12 text-xs-center>
            <v-alert icon="mdi-check-circle" :value="message" type="success">
              {{ message }}
            </v-alert>
          </v-flex>

          <v-flex xs12>
            <v-btn color="primary" @click="request_recovery" :loading="is_loading">
              Send recovery email
            </v-btn>
          </v-flex>

          <v-flex xs12 id="help-link">
            <a href="https://help.bloom.sh" target="_blank" rel="noopener">Help</a>
          </v-flex>
        </v-layout>
      </v-card>
    </v-flex>
  </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class RequestRecoveryForm extends Vue {
  // props
  // data
  is_valid = false;
  is_loading = false;
  email_or_username = '';
  message = '';
  error = '';


  // computed
  // lifecycle
  // watch
  // methods
  async request_recovery() {
    this.error = '';
    this.message = '';
    this.is_loading = true;
    try {
      const payload = { email_or_username: this.email_or_username };
      await api.post(`${api.MYACCOUNT}/v1/recover`, payload);
      // tslint:disable-next-line:max-line-length
      this.message = 'If the Account exists, an email was sent. It may take few minutes to arrive. The link will expire in 30 minutes.';
      // store.commit('set_session', null);
      this.email_or_username = '';
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  lowercase_email_or_username() {
    this.email_or_username = this.email_or_username.toLowerCase();
  }
}
</script>


<style scoped lang="scss">
#main {
  margin-top: 42px;
}

#help-link {
  margin-top: 20px;
}

a {
  text-decoration: none;
}
a:hover {
  text-decoration: underline;
}
</style>
