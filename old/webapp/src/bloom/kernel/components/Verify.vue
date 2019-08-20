<template>
  <v-layout row wrap text-xs-center>
    <v-flex xs12 sm8 md6 offset-sm2 offset-md3 id="main" class="text-xs-center">
      <div class="elevation-0">
        <v-container fluid fill-height>
          <v-layout row wrap text-xs-center>
            <v-flex xs12 v-if="!email">
              <v-progress-circular
                :size="50"
                color="primary"
                indeterminate
                v-if="is_loading"
              ></v-progress-circular>
            </v-flex>
            <v-flex xs12 v-if="email">
              <h2>Check your email</h2>
              <p>
                We've sent a 8 digit confirmation code to {{ email }}.<br/>
                The code will only be valid for 30 minutes.
              </p>
            </v-flex>
            <v-flex xs12 sm6 offset-sm3 v-if="email">
              <v-text-field
              v-model="code"
              label="Your confirmation code"
              :disabled="is_loading"
              outline
              mask="####-####"
              @keyup="check_code_length"
              ></v-text-field>
            </v-flex>
            <v-flex xs12 text-xs-center v-if="error || success">
              <v-alert icon="mdi-alert-circle" :value="error" type="error">
                {{ error }}
              </v-alert>
              <v-alert icon="mdi-check-circle" :value="success" type="success">
                {{ success }}
              </v-alert>
            </v-flex>
            <v-flex xs12 v-if="email">
              <v-btn color="primary" @click="verify" :loading="is_loading">
                Verify
              </v-btn>
            </v-flex>
            <v-flex xs12 class="spam" v-if="email">
              <p>Can't find it? Check your spam folder!</p>
              <v-btn flat color="primary"
              v-if="display_send_new_code"
              @click="send_new_code"
              :loading="is_loading">
                Send a new code
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


const RESEND_TIMEOUT = 15000;

@Component
export default class Verify extends Vue {
  is_loading = false;
  code = '';
  pending_account_id = '';
  email = '';
  error = '';
  success = '';
  display_send_new_code = false;

  created() {
    if (this.$route.query.code) {
      this.code = this.$route.query.code as string;
    }
    if (this.$route.query.id) {
      this.pending_account_id = this.$route.query.id as string;
    }
    if (this.$store.state.pending_account) {
      if (this.$store.state.pending_account.email) {
        this.email = this.$store.state.pending_account.email;
      }
      if (this.$store.state.pending_account.id) {
        this.pending_account_id = this.$store.state.pending_account.id;
      }
    }

    if (!this.pending_account_id && !this.code) {
      router.push({ path: '/register' });
    }

    if (this.pending_account_id) {
      setTimeout(() => {
        this.display_send_new_code = true;
      }, RESEND_TIMEOUT);
    }

    if (this.pending_account_id && this.code) {
      this.verify();
    }
  }


  check_code_length() {
    if (this.code.length === 8) {
      this.verify();
    }
  }

  async verify() {
    this.error = '';
    this.success = '';
    this.display_send_new_code = false;
    const pending_account = {
      code: this.code,
      id: this.pending_account_id,
    };
    try {
      this.is_loading = true;
      const res = await api.post(`${api.MYACCOUNT}/v1/registration/verify`, pending_account);
      this.$store.commit('set_pending_account', pending_account);
      router.push({ path: '/welcome/username', query: { service: this.$route.query.service } });

    } catch (err) {
      this.error = err.message;
      setTimeout(() => {
        this.display_send_new_code = true;
      }, RESEND_TIMEOUT);
    } finally {
      this.is_loading = false;
    }
  }

  async send_new_code() {
    if (!this.pending_account_id) {
      return;
    }
    this.error = '';
    this.success = '';
    const payload = {
      id: this.pending_account_id,
    };
    this.is_loading = true;
    try {
      const res = await api.post(`${api.MYACCOUNT}/v1/registration/new-code`, payload);
      this.success = 'A new code will arrive shortly';
      this.display_send_new_code = false;
      setTimeout(() => {
        this.display_send_new_code = true;
      }, RESEND_TIMEOUT);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
#main {
  margin-top: 42px;
}

.spam {
  margin-top: 20px;
}
.spam {
  p {
    margin-bottom: 10px;
  }
}
</style>
