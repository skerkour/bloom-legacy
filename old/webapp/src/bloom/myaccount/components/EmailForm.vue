<template>
  <v-layout row wrap>

    <v-flex hidden-xs-only sm3>
      <v-subheader>Email</v-subheader>
    </v-flex>
    <v-flex xs12 sm9>
      <v-text-field
      v-model="new_email"
      label="Email"
      type="email"
      @keyup.enter.native="update_email"
      @keyup="lowercase_email"
      ></v-text-field>
    </v-flex>

    <v-flex xs12 text-xs-center v-if="error">
      <v-alert icon="mdi-alert-circle" :value="error" type="error">
        {{ error }}
      </v-alert>
    </v-flex>

    <v-flex class="text-xs-right" xs12>
      <v-btn flat @click="cancel" :disabled="is_loading || !updatable">Cancel</v-btn>
      <v-btn color="primary" @click="update_email" :loading="is_loading" :disabled="!updatable">
        Update
      </v-btn>
    </v-flex>

    <v-dialog
    v-model="confirmation_dialog" width="500">
    <v-card>
      <v-card-title>
        <h2 cass="title">Check your Email</h2>
      </v-card-title>
      <v-card-text>
        <v-container>
          <v-alert icon="mdi-check-circle" :value="dialog_success" type="success">
            Success!
          </v-alert>
          <v-alert icon="mdi-check-circle" :value="dialog_error" type="error">
            {{ dialog_error }}
          </v-alert>
          <p>
            We've sent a 8 digit confirmation code to <b>{{ new_email }}</b> to confirm ownership.
            <br/>
            The code will only be valid for 30 minutes.
          </p>
          <v-text-field
          v-model="verification_code"
          label="Your confirmation code"
          :disabled="is_loading"
          outline
          mask="####-####"
          @keyup="check_verification_code_length"
          ></v-text-field>
        </v-container>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn flat color="primary" @click="close_dialog" :loading="is_loading">
          Cancel
        </v-btn>
        <v-btn
        color="primary"
        :disabled="verification_code.length !== 8"
        :loading="is_loading"
        @click="verify">
          Confirm
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
  </v-layout>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class EmailForm extends Vue {
  // props
  @Prop({ type: String, default: '' }) email!: string;


  // data
  is_loading = false;
  new_email = '';
  error = '';
  dialog_error = '';
  pending_email_id = '';
  confirmation_dialog = false;
  verification_code = '';
  dialog_success = true;


  // computed
  get updatable() {
    return this.email !== this.new_email;
  }


  // lifecycle
  created() {
    this.cancel();
  }


  // watch
  // methods
  async update_email() {
    this.error = '';
    this.is_loading = true;
    try {
      const res = await api.put(`${api.MYACCOUNT}/v1/me/email`, { email: this.new_email });

      this.pending_email_id = res.id;
      this.confirmation_dialog = true;
      setTimeout(() => {
        this.dialog_success = false;
      }, 5000);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  cancel() {
    this.new_email = this.email;
  }

  lowercase_email() {
    this.new_email = this.new_email.toLowerCase();
  }

  check_verification_code_length() {
    if (this.verification_code.length === 8) {
      this.verify();
    }
  }

  async verify() {
    try {
      this.dialog_error = '';
      this.is_loading = true;

      const payload = { id: this.pending_email_id, code: this.verification_code };
      const res = await api.post(`${api.MYACCOUNT}/v1/me/email/verify`, payload);
      this.close_dialog();
      this.$toast.success('Success', { icon: 'mdi-check-circle' });
      this.$emit('update', res.email);
      this.$store.commit('set_account', res);
    } catch (err) {
      this.dialog_error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  close_dialog() {
    this.confirmation_dialog = false;
    this.verification_code = '';
    this.dialog_error = '';
    this.dialog_success = false;
  }
}
</script>
