<template>
  <v-container fluid>
    <v-alert type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar flat dense>
      <v-spacer />
      <v-menu offset-y>
        <template v-slot:activator="{ on }">
          <v-btn color="primary" class="new-btn" v-on="on">
            <v-icon left>mdi-plus</v-icon>New
          </v-btn>
        </template>
        <v-list>
          <v-list-item>
            <v-list-item-icon>
              <v-icon>mdi-folder-plus</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Folder</v-list-item-title>
          </v-list-item>
          <v-divider />
          <v-list-item>
            <v-list-item-icon>
              <v-icon>mdi-file-upload</v-icon>
            </v-list-item-icon>
            <v-list-item-title>File upload</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>

      <v-tooltip bottom>
        <template v-slot:activator="{ on: tooltip }">
          <v-btn icon v-on="tooltip">
            <v-icon>mdi-folder-move</v-icon>
          </v-btn>
        </template>
        <span>Move to...</span>
      </v-tooltip>

      <v-tooltip bottom>
        <template v-slot:activator="{ on: tooltip }">
          <v-btn icon v-on="tooltip">
            <v-icon>mdi-delete</v-icon>
          </v-btn>
        </template>
        <span>Delete</span>
      </v-tooltip>

      <v-tooltip bottom>
        <template v-slot:activator="{ on: tooltip }">
          <v-btn icon v-on="tooltip">
            <v-icon>mdi-download</v-icon>
          </v-btn>
        </template>
        <span>Download</span>
      </v-tooltip>
    </v-toolbar>

    <blm-drive-table-files :files="files" />

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import FilesTable from '../components/files_table.vue';

@Component({
  components: {
    'blm-drive-table-files': FilesTable,
  },
})
export default class Index extends Vue {
  // props
  // data
  error = '';
  files = [];

  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
