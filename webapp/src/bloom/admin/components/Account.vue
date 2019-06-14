<template>
  <div>
    <v-container>
      <v-layout row wrap justify-left>

        <v-flex xs12>
          <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
          {{ error }}
        </v-alert>
        </v-flex>

        <v-flex xs12 v-if="is_loading">
          <v-card class="elevation-0">
            <v-layout row wrap justify-center>
            <v-progress-circular
              :size="50"
              color="primary"
              indeterminate
            ></v-progress-circular>
          </v-layout>
          </v-card>
        </v-flex>

        <v-flex xs12 v-if="account">
          <p>
            {{ account.id }}
          </p>
          <p>
            {{ account.username }}
          </p>
          <v-btn color="success" v-if="account.is_disabled" @click="enable_account">
            Activate
          </v-btn>
          <v-btn v-else color="error" @click="disable_account">
            Disable
          </v-btn>
        </v-flex>
        <v-flex xs12 v-if="account" class="text-xs-center">
          <v-btn color="error" @click="delete_account(account)">
            Delete
          </v-btn>
        </v-flex>
      </v-layout>
    </v-container>
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import router from '@/bloom/kernel/router';


@Component
export default class Account extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  account: any = null;
  selected: any[] = [];

  // computed
  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    try {
      this.account = await api.get(`${api.ADMIN}/v1/accounts/${this.$route.params.account_id}`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async disable_account() {
    this.error = '';
    this.is_loading = true;
    try {
      await api.post(`${api.ADMIN}/v1/accounts/${this.$route.params.account_id}/disable`);
      this.account.is_disabled = true;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async enable_account() {
    this.error = '';
    this.is_loading = true;
    try {
      await api.post(`${api.ADMIN}/v1/accounts/${this.$route.params.account_id}/enable`);
      this.account.is_disabled = false;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async delete_account(account: any) {
    if (!confirm(`Are you sure you want to delete ${account.username} (${account.id})`)) {
      return;
    }
    if (!confirm(`Are you REALLY SURE you want to DELETE ${account.username} (${account.id})`)) {
      return;
    }
    this.error = '';
    this.is_loading = true;
    try {
      await api.delete(`${api.ADMIN}/v1/accounts/${this.$route.params.account_id}`);
      router.push({ path: '/admin' });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
</style>
