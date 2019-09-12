<template>
<v-container>
  <v-layout row wrap text-xs-center>
    <v-flex xs12 sm8 md6 offset-sm2 offset-md3 id="main" class="text-xs-center">
      <v-card class="elevation-0">
        <div>
          <h2>Reset your password</h2>
        </div>
        <v-form ref="recoveryform" v-model="is_valid">
          <v-layout row wrap text-xs-center>
            <v-flex xs12 sm8 md6 offset-sm2 offset-md3>
              <v-text-field
              v-model="password"
              label="New password"
              :disabled="is_loading"
              :rules="password_rules"
              :type="show_password ? 'text' : 'password'"
              :append-icon="show_password ? 'mdi-eye-off' : 'mdi-eye'"
              @click:append="show_password = !show_password"
              ></v-text-field>
            </v-flex>

            <v-flex xs12 sm8 md6 offset-sm2 offset-md3>
              <v-text-field
              v-model="password_verification"
              label="Verify password"
              :disabled="is_loading"
              :type="show_password_verification ? 'text' : 'password'"
              :append-icon="show_password_verification ? 'mdi-eye-off' : 'mdi-eye'"
              @click:append="show_password_verification = !show_password_verification"
              @keyup.enter.native="recover"
              ></v-text-field>
            </v-flex>

            <v-flex xs12 text-xs-center>
              <v-alert icon="mdi-alert-circle" :value="error" type="error">
                {{ error }}
              </v-alert>
            </v-flex>

            <v-flex xs12 text-xs-center>
              <v-alert :value="message" type="success">
                {{ message }}
              </v-alert>
            </v-flex>

            <v-flex xs12>
              <v-btn color="primary" @click="recover" :loading="is_loading">
                Reset Password
              </v-btn>
            </v-flex>

            <v-flex xs12 id="support-link">
              <a href="https://bloom.sh/help" target="_blank" rel="noopener">Support</a>
            </v-flex>
          </v-layout>
        </v-form>
      </v-card>
    </v-flex>
  </v-layout>
</v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import router from '@/bloom/kernel/router';

@Component
export default class RecoveryForm extends Vue {
  // props
  // data
  is_valid = false;
  is_loading = false;
  password = '';
  password_rules = [
    (v: string) => !!v || 'Password is required',
  ];
  show_password = false;
  password_verification = '';
  show_password_verification = false;
  error = '';
  id = '';
  token = '';
  message = '';


  // computed
  // lifecycle
  created() {
    if (this.$route.query.token) {
      this.token = this.$route.query.token as string;
    }
    if (this.$route.query.id) {
      this.id = this.$route.query.id as string;
    }

    if (!this.id || !this.token) {
      router.push({ path: '/sign-in' });
    }
  }


  // watch
  // methods
  async recover() {
    this.error = '';
    if ((this.$refs.recoveryform as any).validate()) {
      if (this.password_verification !== this.password) {
        this.error = 'Passwords does not match';
        return;
      }
      this.is_loading = true;
      const payload = {
        id: this.id,
        new_password: this.password,
        token: this.token,
      };
      try {
        const res = await api.put(`${api.MYACCOUNT}/v1/recover`, payload);
        api.sign_in(res);
        router.push({ path: '/' });
      } catch (err) {
        this.error = err.message;
      } finally {
        this.is_loading = false;
      }
    }
  }
}
</script>


<style scoped lang="scss">
#main {
  margin-top: 42px;
}

#support-link {
  margin-top: 20px;
}
</style>
