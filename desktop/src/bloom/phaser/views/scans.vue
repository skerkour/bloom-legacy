<template>
  <v-container fluid>
    <v-alert type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar flat dense>
      <v-spacer />
      <v-btn color="primary" class="new-btn" @click="openNewScanDialog">
        <v-icon left>mdi-plus</v-icon>New scan
      </v-btn>
    </v-toolbar>


    <v-data-table
    :headers="headers"
    :items="scans"
    item-key="id"
    hide-default-footer
    :loading="isLoading"
    class="text-left">
      <template slot="no-data">
        <p class="text-center">
          No scan yet
        </p>
      </template>
      <template v-slot:item="{ item }">
        <td>{{ item.name }}</td>
        <td>{{ item.target }}</td>
        <td>{{ item.profile }}</td>
        <td>
          <span v-if="item.state === State.Queued">
            Queued
          </span>
          <div v-else-if="item.state === State.Scanning">
            <v-progress-circular
              indeterminate
              :size="18"
              :width="2"
              color="primary"
            /> Running
          </div>
          <span v-else-if="item.last">
            {{ item.last | calendar }}
          </span>
          <span v-else>
            never
          </span>

        </td>
        <td>{{ item.findings }}</td>
        <td class="justify-left layout px-0">

          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-btn
                v-if="item.last"
                text
                icon
                small
                v-on="on"
                :to="`/phaser/scans/${item.id}/reports`"
                :disabled="!item.last"
              >
                <v-icon small>mdi-magnify</v-icon>
              </v-btn>
            </template>
            <span v-if="item.last">See reports</span>
            <span v-else>No report yet</span>
          </v-tooltip>

          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-btn
                text
                icon
                small
                v-on="on"
                @click="queueScan(item)"
                v-if="item.state === State.Waiting"
              >
                <v-icon small>mdi-play</v-icon>
              </v-btn>
            </template>
            <span>Run now</span>
          </v-tooltip>

          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-btn
                text
                icon
                small
                color="grey darken-1"
                v-on="on"
                @click="cancelScan(item)"
                v-if="item.state !== State.Waiting"
              >
                <v-icon small>mdi-stop</v-icon>
              </v-btn>
            </template>
            <span>Cancel Scan</span>
          </v-tooltip>

          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
            <v-btn
              text
              icon
              small
              color="grey darken-1"
              v-on="on"
              @click="deleteScan(item)"
            >
              <v-icon small>mdi-delete</v-icon>
            </v-btn>
            </template>
            <span>Delete</span>
          </v-tooltip>

        </td>
      </template>
    </v-data-table>

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { State } from '../models/scan';

@Component
export default class Index extends Vue {
  // props
  // data
  isLoading = false;
  error = '';
  scans = [
    {
      id: '1',
      last: {
        id: '1',
      },
      state: State.Scanning,
      name: 'kerkour.fr',
      target: 'kerkour.fr',
      profile: 'Application',
      findings: 42,
    },
  ];
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
    { text: 'Findings', value: 'findings', sortable: false },
    { text: 'Actions', sortable: false },
  ];


  // computed
  get State(): typeof State {
    return State;
  }

  // lifecycle
  // watch
  // methods
  openNewScanDialog() {
  }

  queueScan() {
  }

  cancelScan() {
  }

  deleteScan() {
  }
}
</script>


<style lang="scss" scoped>
.new-btn {
  height: 36px !important;
  border-radius: 4px;
}
</style>
