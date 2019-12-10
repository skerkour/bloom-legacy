<template>
  <v-container fluid>
    <v-alert type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar elevation="0">
      <v-spacer></v-spacer>

      <v-toolbar-items>
        <v-btn outlined color="primary" class="new-btn" @click="openNewScanDialog">
          <v-icon left>mdi-plus</v-icon>New scan
        </v-btn>
      </v-toolbar-items>

    </v-toolbar>


    <v-data-table
    :headers="headers"
    :items="scans"
    item-key="id"
    hide-default-footer
    :loading="isLoading">
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
        <td>{{ props.item.findings }}</td>
        <td class="justify-left layout px-0">

          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-btn
                v-if="props.item.last"
                text
                icon
                small
                color="grey darken-1"
                v-on="on"
                :to="`/platform/phaser/scans/${props.item.id}/reports`"
              >
                <v-icon small>mdi-magnify</v-icon>
              </v-btn>
            </template>
            <span>See reports</span>
          </v-tooltip>

           <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-btn
                v-if="!props.item.last"
                text
                icon
                small
                color="grey darken-1"
                v-on="on"
                disabled
              >
                <v-icon small>mdi-magnify</v-icon>
              </v-btn>
            </template>
            <span>No report yet</span>
          </v-tooltip>

          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-btn
                text
                icon
                small
                color="grey darken-1"
                v-on="on"
                @click="queue_scan(props.item)"
                v-if="props.item.state === State.Waiting"
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
                @click="cancel_scan(props.item)"
                v-if="props.item.state !== State.Waiting"
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
              @click="delete_scan(props.item)"
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

@Component
export default class Index extends Vue {
  // props
  // data
  isLoading = false;
  error = '';
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
    { text: 'Findings', value: 'findings', sortable: false },
    { text: 'Actions', sortable: false },
  ];
  // computed
  // lifecycle
  // watch
  // methods
  openNewScanDialog() {
  }
}
</script>


<style lang="scss" scoped>
.new-btn {
  height: 36px !important;
  border-radius: 4px;
}
</style>
