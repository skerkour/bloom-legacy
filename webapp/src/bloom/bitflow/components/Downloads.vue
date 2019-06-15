<template>
  <div>
    <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
      {{ error }}
    </v-alert>

  <v-toolbar flat dense class="hidden-sm-and-down">
    <v-spacer></v-spacer>
    <v-btn outline color="primary" class="add-btn" @click="open_add_dialog">
      <v-icon left>mdi-plus</v-icon>Add
    </v-btn>
    <v-toolbar-items>
      <v-tooltip bottom>
        <v-btn slot="activator" icon :disabled="selected.length === 0" @click="remove">
          <v-icon color="grey darken-1">mdi-delete</v-icon>
        </v-btn>
        <span>Remove</span>
      </v-tooltip>
    </v-toolbar-items>
  </v-toolbar>

  <v-data-table
  :headers="headers"
  :items="downloads"
  item-key="id"
  hide-actions
  :loading="is_loading"
  v-model="selected">
  <template slot="no-data">
    <p class="text-xs-center">
      No download.
    </p>
  </template>
  <template slot="items" slot-scope="props">
    <tr class="pointer"
    :active="props.selected"
    @click="props.selected = !props.selected">

    <td class="text-xs-left">
      <span>{{ props.item.name | truncate }}</span>
    </td>
    <td class="text-xs-left">
      <v-progress-linear v-if="props.item.status === DownloadStatus.Downloading" :indeterminate="true" />
      <v-progress-linear color="success" value="100" v-else-if="props.item.status === DownloadStatus.Completed" />
      <span v-else>{{ props.item.status }}</span>
    </td>
    <td class="justify-left layout px-0">
      <v-tooltip bottom>
        <v-btn
          flat
          icon
          small
          color="grey darken-1"
          slot="activator"
          to="/drive"
          :disabled="props.item.status !== DownloadStatus.Success"
        >
          <v-icon small>mdi-folder</v-icon>
        </v-btn>
        <span>Open in Drive</span>
      </v-tooltip>
    </td>
  </tr>
</template>
</v-data-table>


  <v-btn
  @click="open_add_dialog"
  color="red"
  dark
  fab
  fixed
  bottom
  right
  class="hidden-md-and-up"
  >
  <v-icon>mdi-plus</v-icon>
  </v-btn>


  <v-dialog
      v-model="show_add_dialog"
      max-width="400px"
      @keydown.esc="close_add_dialog"
      scrollable
    >
      <v-card>
        <v-card-title
          class="headline blue darken-1 white--text"
          primary-title
        >
          New download
        </v-card-title>

        <v-card-text class="text-xs-center">
          <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
            {{ error }}
          </v-alert>
          <p>
            Download size is limited to 8GB.
          </p>
          <v-text-field
          placeholder="Paste your link or your magnet link here."
          outline
          v-model="download_url"
          :disabled="is_loading"
          @keyup.enter.native="queue_download"
          ></v-text-field>
        </v-card-text>

        <v-divider></v-divider>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="blue darken-1" flat @click.native="close_add_dialog">Cancel</v-btn>
          <v-btn color="primary"@click.native="queue_download" :disabled="!can_download">
            Download
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import { DownloadStatus } from '../models/Download';


@Component
export default class Downloads extends Vue {
  // props
  // data
  error = '';
  download_url = '';
  is_loading = false;
  show_add_dialog = false;
  downloads: any[] = [];
  selected: any[] = [];
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Name',
      value: 'name',
      width: '50%',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Progress',
      value: 'progress',
    },
    { text: 'Actions', sortable: false },
  ];
  interval: any | null = null;


  // computed
  get DownloadStatus(): typeof DownloadStatus {
    return DownloadStatus;
  }

  get can_download() {
    return !this.is_loading && this.download_url.length !== 0;
  }


  // lifecycle
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


  // watch
  // methods
  open_add_dialog() {
    this.show_add_dialog = true;
  }

  close_add_dialog() {
    this.show_add_dialog = false;
    this.download_url = '';
  }

  async fetch_data(loading?: boolean) {
    if (loading !== false) {
      this.error = '';
      this.is_loading = true;
    }
    try {
      this.downloads = await api.get(`${api.BITFLOW}/v1/downloads`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async remove() {
    this.error = '';
    const downloads = this.selected.map((f: any) => f.id);
    this.is_loading = true;
    const paylaod = {
      downloads,
    };
    try {
      await api.post(`${api.BITFLOW}/v1/downloads/remove`, paylaod);
      this.downloads = this.downloads.filter((f: any) => downloads.indexOf(f.id) === -1);
      this.selected = [];
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async queue_download() {
    this.error = '';
    this.is_loading = true;
    const payload = {
      url: this.download_url,
    };
    try {
      const download = await api.post(`${api.BITFLOW}/v1/downloads`, payload);
      this.close_add_dialog();
      this.downloads.push(download);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

}
</script>


<style scoped lang="scss">
/* Add download button */
.add-btn {
  height: 36px !important;
}
</style>
