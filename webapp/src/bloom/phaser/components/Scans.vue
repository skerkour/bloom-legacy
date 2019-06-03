<template>
  <div>
    <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
      {{ error }}
    </v-alert>

    <v-toolbar flat dense color="white">
      <v-spacer />
      <v-toolbar-items>
        <v-btn outline color="primary" class="new-btn" @click="open_new_scan_dialog">
          <v-icon left>mdi-plus</v-icon>New scan
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-data-table
    :headers="headers"
    :items="scans"
    item-key="id"
    hide-actions
    :loading="is_loading">
      <template slot="no-data">
        <p class="text-xs-center">
          No scan yet
        </p>
      </template>
      <template slot="items" slot-scope="props">
        <td>{{ props.item.name }}</td>
        <td>{{ props.item.target }}</td>
        <td>{{ props.item.profile }}</td>
        <td>
          <span v-if="props.item.state === State.Queued">
            Queued
          </span>
          <div v-else-if="props.item.state === State.Scanning">
            <v-progress-circular
              indeterminate
              :size="18"
              :width="2"
              color="primary"
            /> Running
          </div>
          <span v-else-if="props.item.last">
            {{ props.item.last | calendar }}
          </span>
          <span v-else>
            never
          </span>

        </td>
        <!-- <td></td>
        <td></td> -->
        <td>{{ props.item.findings }}</td>
        <td class="justify-left layout px-0">

          <v-tooltip bottom>
            <v-btn
              v-if="props.item.last"
              flat
              icon
              small
              color="grey darken-1"
              slot="activator"
              :to="`/platform/phaser/scans/${props.item.id}/reports`"
            >
              <v-icon small>mdi-magnify</v-icon>
            </v-btn>
            <span>See reports</span>
          </v-tooltip>

           <v-tooltip bottom>
            <v-btn
              v-if="!props.item.last"
              flat
              icon
              small
              color="grey darken-1"
              slot="activator"
              disabled
            >
              <v-icon small>mdi-magnify</v-icon>
            </v-btn>
            <span>No report yet</span>
          </v-tooltip>

          <v-tooltip bottom>
            <v-btn
              flat
              icon
              small
              color="grey darken-1"
              slot="activator"
              @click="queue_scan(props.item)"
              v-if="props.item.state === State.Waiting"
            >
              <v-icon small>mdi-play</v-icon>
            </v-btn>
            <span>Run now</span>
          </v-tooltip>

          <v-tooltip bottom>
            <v-btn
              flat
              icon
              small
              color="grey darken-1"
              slot="activator"
              @click="cancel_scan(props.item)"
              v-if="props.item.state !== State.Waiting"
            >
              <v-icon small>mdi-stop</v-icon>
            </v-btn>
            <span>Cancel Scan</span>
          </v-tooltip>

          <v-tooltip bottom>
            <v-btn
              flat
              icon
              small
              color="grey darken-1"
              slot="activator"
              @click="delete_scan(props.item)"
            >
              <v-icon small>mdi-delete</v-icon>
            </v-btn>
            <span>Delete</span>
          </v-tooltip>

        </td>
      </template>
    </v-data-table>

    <blm-phaser-dialog-new-scan
    :visible="new_scan_dialog"
    @close="close_new_scan_dialog"
    @create="scan_created"/>

    <blm-phaser-dialog-add-payment
    :visible="add_payment_dialog"
    @close="close_add_payment_dialog" />
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import router from '@/bloom/kernel/router';
import NewScanDialog from './NewScanDialog.vue';
import AddPaymentDialog from './AddPaymentDialog.vue';
import { State } from '../models/Scan';


@Component({
  components: {
    'blm-phaser-dialog-add-payment': AddPaymentDialog,
    'blm-phaser-dialog-new-scan': NewScanDialog,
  },
})
export default class Scans extends Vue {
  error = '';
  new_scan_dialog = false;
  add_payment_dialog = false;
  is_loading = false;
  scans: any[] = [];
  interval: any = null;
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Name',
      value: 'name',
    },
    { text: 'Target', value: 'target', sortable: false },
    { text: 'Profile', value: 'profile', sortable: true },
    { text: 'Last run', value: 'updated_at', sortable: true },
    // { text: 'Next run', value: 'updated_at', sortable: true },
    { text: 'Findings', value: 'findings', sortable: false },
    { text: 'Actions', sortable: false },
  ];

  get State(): typeof State {
    return State;
  }


  created() {
    this.fetch_data();
    this.interval = setInterval(() => {
      this.fetch_data(false);
    }, 10000);
  }

  destroyed() {
    if (this.interval) {
      clearInterval(this.interval);
      this.interval = null;
    }
  }


  open_new_scan_dialog() {
    if (this.scans.length !== 0) {
      this.add_payment_dialog = true;
    } else {
      this.new_scan_dialog = true;
    }
  }

  close_new_scan_dialog() {
    this.new_scan_dialog = false;
  }

  close_add_payment_dialog() {
    this.add_payment_dialog = false;
  }

  scan_created(scan: any) {
    this.scans.push(scan);
  }

  async fetch_data(loading?: boolean) {
    if (loading !== false) {
      this.error = '';
      this.is_loading = true;
    }
    try {
      this.scans = await api.get(`${api.PHASER}/v1/scans`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async delete_scan(scan: any) {
    this.error = '';
    this.is_loading = true;
    try {
      await api.delete(`${api.PHASER}/v1/scans/${scan.id}`);
      this.scans = this.scans.filter((s: any) => s.id !== scan.id);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async cancel_scan(scan: any) {
    this.error = '';
    this.is_loading = true;
    try {
      await api.post(`${api.PHASER}/v1/scans/${scan.id}/cancel`);
      scan.state = State.Waiting;
      const index = this.scans.map((s: any) => s.id).indexOf(scan.id);
      this.$set(this.scans, index, scan);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async queue_scan(scan: any) {
    this.error = '';
    this.is_loading = true;
    try {
      await api.post(`${api.PHASER}/v1/scans/${scan.id}/queue`);
      scan.state = State.Queued;
      const index = this.scans.map((s: any) => s.id).indexOf(scan.id);
      this.$set(this.scans, index, scan);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  goto_reports(scan: any) {
    if (scan.reports.length === 0) {
      return;
    }
    const report_id = scan.reports[0].id;
    router.push({ path: `/platform/phaser/${scan.id}/reports/${report_id}` });
  }
}
</script>


<style scoped lang="scss">
.new-btn {
  height: 36px !important;
}
</style>
