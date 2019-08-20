<template>
  <v-container fluid grid-list-xs>
    <v-layout row wrap justify-left>

      <v-flex xs12 text-xs-center v-if="error">
        <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
          {{ error }}
        </v-alert>
      </v-flex>


      <v-flex xs12 sm4 lg3>
        <v-card>
          <v-card-title class="headline">
            Bloom
          </v-card-title>
          <v-divider />

          <v-card-text class="text-xs-center">
            {{ config.VERSION }}
          </v-card-text>
        </v-card>
      </v-flex>

      <v-flex xs12 sm4 lg3>
        <router-link to="/admin/accounts">
        <v-hover>
        <v-card slot-scope="{ hover }" :class="`elevation-${hover ? 4 : 1}`">
          <v-card-title class="headline">
            Accounts
          </v-card-title>
          <v-divider />

          <v-card-text class="text-xs-center">
            {{ total }}
          </v-card-text>
        </v-card>
        </v-hover>
        </router-link>
      </v-flex>

    </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import router from '@/bloom/kernel/router';
import config from '@/config';


@Component
export default class Account extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  total = 0;
  config = config;

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
      const res = await api.get(`${api.ADMIN}/v1/accounts`);
      this.total = res.total;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
a {
  text-decoration: none;
}
</style>
