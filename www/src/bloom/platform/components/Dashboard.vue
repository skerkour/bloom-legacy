<template>
  <v-container>
    <v-layout row wrap>
      <v-flex xs12 text-xs-center v-if="error">
        <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
          {{ error }}
        </v-alert>
      </v-flex>
      <v-flex xs12 sm4 lg3>
        <router-link to="/platform/phaser">
        <v-hover>
        <v-card slot-scope="{ hover }" :class="`elevation-${hover ? 4 : 1}`">
          <v-card-title class="grey--text text--darken-2 headline">
            <img src="/kernel/static/imgs/logos/phaser.svg" height="32"/>&nbsp;
            Phaser
          </v-card-title>
          <v-divider />

          <v-card-text class="text-xs-center">
            {{ scans.length }} Scan{{ scans.length === 1 ? '' : 's' }}
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


@Component
export default class Dashboard extends Vue {
  // props
  // data
  scans: any[] = [];
  error = '';
  is_loading = false;
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
      this.scans = await api.get(`${api.PHASER}/v1/scans`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
.v-card {
  border-radius: 4px;
}

a {
  text-decoration: none;
}
</style>
