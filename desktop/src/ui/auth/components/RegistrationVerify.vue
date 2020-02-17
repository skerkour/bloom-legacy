<template>
  <v-container grid-list-xl text-xs-center>
    <v-layout row wrap text-xs-center>

      <v-flex xs12 v-if="!email">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
          v-if="isLoading"
        />
      </v-flex>
      <v-flex xs12 v-if="email">
        <h2>Check your email</h2>
        <p class="mt-3">
          We've sent a 8 digit confirmation code to {{ email }}.<br/>
          The code will only be valid for 30 minutes.
        </p>
      </v-flex>

      <v-flex xs12 sm6 offset-sm3 v-if="email">
        <v-text-field
          v-model="code"
          label="Your confirmation code"
          :disabled="isLoading"
          outlined
          v-mask="codeMask"
          @keyup="checkCodeLength"
        />
      </v-flex>

      <v-flex xs12 text-xs-center v-if="error !== ''|| success !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
        <v-alert icon="mdi-check-circle" type="success" :value="success !== ''">
          {{ success }}
        </v-alert>
      </v-flex>

      <v-flex xs12 v-if="email">
        <v-btn color="primary" @click="verify" :loading="isLoading" :disabled="!canVerify">
          Verify
        </v-btn>
      </v-flex>

      <v-flex xs12 class="spam" v-if="email">
        <p>Can't find it? Check your spam folder!</p>
        <v-btn text color="primary"
          v-if="showSendNewCode"
          @click="sendNewCode"
          :loading="isLoading">
          Send a new code
        </v-btn>
      </v-flex>

    </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { mask } from 'vue-the-mask';
import core from '@/core';
import { VerifyRegistration, Method } from '@/core/users';


const RESEND_TIMEOUT = 42_000;
const CODE_LENGTH = 9;

@Component({
  directives: {
    mask,
  },
})
export default class RegistrationVerify extends Vue {
  // props
  // data
  isLoading = false;
  isValid = false;
  error = '';
  code = '';
  pendingAccountId = '';
  email = '';
  success = '';
  showSendNewCode = false;
  codeMask = 'XXXX-XXXX';

  // computed
  get canVerify(): boolean {
    return this.code.length === CODE_LENGTH;
  }

  // lifecycle
  created() {
    // no longer relevant because as a native app, we do not rely on links
    // if (this.$route.query.code) {
    //   this.code = this.$route.query.code as string;
    // }
    // if (this.$route.query.id) {
    //   this.pendingAccountId = this.$route.query.id as string;
    // }
    if (this.$store.state.pendingAccount) {
      this.pendingAccountId = this.$store.state.pendingAccount.id;
      this.email = this.$store.state.pendingAccount.email;
    } else {
      this.$router.push({ path: '/auth/registration/start' });
    }

    if (this.pendingAccountId) {
      setTimeout(() => {
        this.showSendNewCode = true;
      }, RESEND_TIMEOUT);
    }

    if (this.pendingAccountId && this.code) {
      this.verify();
    }
  }

  // watch
  // methods
  async verify() {
    this.error = '';
    this.success = '';
    this.showSendNewCode = false;
    this.isLoading = true;
    const { code } = this;
    const params: VerifyRegistration = {
      id: this.pendingAccountId,
      code,
    };
    try {
      await core.call(Method.VerifyRegistration, params);
      this.$router.push({ path: '/auth/registration/complete', params: { pendingAccountId: this.pendingAccountId } });
    } catch (err) {
      this.error = err.message;
      setTimeout(() => {
        this.showSendNewCode = true;
      }, RESEND_TIMEOUT);
    } finally {
      this.isLoading = false;
    }
  }

  checkCodeLength() {
    if (this.canVerify) {
      this.verify();
    }
  }

  async sendNewCode() {
    if (!this.pendingAccountId) {
      return;
    }
    // this.error = '';
    // this.success = '';
    // const payload = {
    //   id: this.pendingAccountId,
    // };
    // this.isLoading = true;
    // try {
    //   const res = await api.post(`${api.MYACCOUNT}/v1/registration/new-code`, payload);
    //   this.success = 'A new code will arrive shortly';
    this.showSendNewCode = false;
    setTimeout(() => {
      this.showSendNewCode = true;
    }, RESEND_TIMEOUT);
    // } catch (err) {
    //   this.error = err.message;
    // } finally {
    //   this.is_loading = false;
    // }
  }
}
</script>


<style lang="scss" scoped>
</style>
