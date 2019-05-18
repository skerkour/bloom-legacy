<template>
  <v-container>
    <v-layout row wrap justify-center>
      <v-flex xs12 md10 v-if="sessions.length !== 0">
        <blm-myaccount-devices
        :sessions="sessions"
        :current="current_session"
        @update="update_sessions"/>
      </v-flex>
      <v-flex xs12 v-else class="text-xs-center">
          <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
          ></v-progress-circular>
      </v-flex>
    </v-layout>
  </v-container>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import DevicesList from '../components/Devices.vue';
const { log } = require('@bloom42/astro');


@Component({
  components: {
    'blm-myaccount-devices': DevicesList,
  },
})
export default class Devices extends Vue {
  private current_session: object | null = null;
  private is_loading = false;
  private sessions: any[] = [];

  created() {
    this.current_session = this.$store.state.session;
    this.fetch_data();
  }


  private update_sessions(session_id: string) {
    this.sessions = this.sessions.filter((s) => s.id !== session_id);
  }

  private async fetch_data() {
    try {
      this.is_loading = true;
      const res = await api.get(`${api.MYACCOUNT}/v1/me/sessions`);
      this.sessions = res.map((session: any) => {
        session.is_loading = false;
        return session;
      });
     } catch (err) {
       log.error(err);
     } finally {
       this.is_loading = false;
     }
  }
}
</script>
