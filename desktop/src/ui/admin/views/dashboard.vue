<template>
  <v-container>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-row v-if="data">
      <v-col cols="12" sm="6" md="3">
        <v-card outlined>
          <v-card-title>Bloom</v-card-title>
          <v-card-text>
            Server: v{{ data.metadata.version }}
            | {{ data.metadata.os }}-{{ data.metadata.arch }}
          </v-card-text>
        </v-card>
      </v-col>


      <v-col cols="12" sm="6" md="3">
        <router-link to="/admin/users">
          <v-card outlined>
            <v-card-title>Users</v-card-title>
            <v-card-text>{{ totalUsers }}</v-card-text>
          </v-card>
        </router-link>
      </v-col>

    </v-row>

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import * as models from '@/api/models';
import core from '@/core';
import { Method as AdminMethod } from '@/core/admin';

@Component
export default class Dashboard extends Vue {
  // props
  // data
  data: models.DashboardData | null = null;
  isLoading = false;
  error = '';
  totalUsers = 100000;

  // computed
  // lifecycle
  created() {
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.isLoading = true;

    try {
      this.data = await core.call(AdminMethod.FetchDashBoardData, core.Empty);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
