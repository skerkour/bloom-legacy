<template>
  <v-container fluid>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar flat dense id="blm-downloads-toolbar">
      <v-spacer />
      <v-btn color="primary" @click="addDownload">
        <v-icon left>mdi-plus</v-icon>New Download
      </v-btn>
    </v-toolbar>


    <v-data-table
      :headers="headers"
      :items="downloads"
      item-key="id"
      :loading="isLoading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No downloads.
        </p>
      </template>

      <template v-slot:item="{ item }">
        <tr class="blm-pointer">
          <td class="text-xs-left">
            <span>{{ item.name | truncate }}</span>
          </td>
          <td class="text-xs-left">
            <v-progress-linear
              v-if="item.status === DownloadStatus.Downloading" :indeterminate="true" />
            <v-progress-linear color="success" value="100"
              v-else-if="item.status === DownloadStatus.Completed" />
            <span v-else>{{ item.status }}</span>
          </td>
          <td class="justify-left layout px-0">
            <v-tooltip bottom>
              <template v-slot:activator="{ on }">
                <v-btn icon @click="removeDownload(item)" v-on="on">
                  <v-icon color="grey darken-1">mdi-delete</v-icon>
                </v-btn>
              </template>
              <span>Remove</span>
            </v-tooltip>
            <v-tooltip bottom>
              <template v-slot:activator="{ on }">
                <v-btn
                  text
                  icon
                  small
                  color="grey darken-1"
                  to="/drive"
                  v-on="on"
                  :disabled="item.status !== DownloadStatus.Success"
                >
                  <v-icon small>mdi-folder</v-icon>
                </v-btn>
              </template>
              <span>Open in Drive</span>
            </v-tooltip>
          </td>
        </tr>
      </template>
    </v-data-table>


  <blm-bitflow-dialog-new-download
    :visible="isNewDownloadDialogVisible"
    @closed="closeNewDownloadDialog"
  />

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import NewDownloadDialog from '../components/new_download_dialog.vue';


@Component({
  components: {
    'blm-bitflow-dialog-new-download': NewDownloadDialog,
  },
})
export default class Downloads extends Vue {
  // props
  // data
  error = '';
  isLoading = false;
  downloads = [];
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
  isNewDownloadDialogVisible = false;

  // computed
  // lifecycle
  // watch
  // methods
  addDownload() {
    this.isNewDownloadDialogVisible = true;
  }

  removeDownload() {
  }

  closeNewDownloadDialog() {
    this.isNewDownloadDialogVisible = false;
  }
}
</script>


<style lang="scss" scoped>
#blm-downloads-toolbar {
  border-bottom: 1px solid rgba($color: #000000, $alpha: 0.1);
}
</style>
