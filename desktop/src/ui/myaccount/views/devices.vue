<template>
  <v-container fluid>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>
    <v-row justify="center">
      <v-col cols="12" sm="10" lg="9">
        <blm-myaccount-table-devices :loading="isLoading" :devices="devices"
          :current="$store.state.session" @revoked="onSessionRevoked" />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import DevicesTable from '../components/devices_table.vue';
import * as models from '@/api/models';
import core from '@/core';
import { Method } from '@/core/users';


@Component({
  components: {
    'blm-myaccount-table-devices': DevicesTable,
  },
})
export default class Devices extends Vue {
  // props
  // data
  isLoading = false;
  error = '';
  me: models.User | null = null;

  // computed
  get devices(): models.Session[] {
    if (this.me) {
      return this.me.sessions!.edges!.map((edge: models.Maybe<models.SessionEdge>) => edge!.node!);
    }
    return [];
  }

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
      this.me = await core.call(Method.FetchMySessions, core.Empty);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  onSessionRevoked(session: models.Session) {
    this.me!.sessions!.edges = this.me!.sessions!.edges!
      .filter((edge: models.Maybe<models.SessionEdge>) => edge!.node!.id !== session.id);
  }
}
</script>


<style lang="scss" scoped>
</style>
