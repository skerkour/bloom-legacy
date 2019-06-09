<template>
  <div>
    <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
      {{ error }}
    </v-alert>

  <v-toolbar flat dense color="white">
    <v-breadcrumbs :items="current_folder.clean_path" v-if="current_folder" class="text-truncate">
      <v-icon slot="divider">mdi-chevron-right</v-icon>
    </v-breadcrumbs>
    <v-spacer></v-spacer>
    <v-toolbar-items>


      <v-menu offset-y>
      <v-btn outline color="primary" class="new-btn hidden-sm-and-down" slot="activator">
        <v-icon left>mdi-plus</v-icon>New
      </v-btn>
      <v-list>

        <v-list-tile @click="open_new_folder_dialog">
          <v-list-tile-action>
            <v-icon>mdi-folder-plus</v-icon>
          </v-list-tile-action>
          <v-list-tile-content>
            <v-list-tile-title>Folder</v-list-tile-title>
          </v-list-tile-content>
        </v-list-tile>

          <v-divider></v-divider>

          <v-list-tile @click="upload_files">
            <v-list-tile-action>
              <v-icon>mdi-file-upload</v-icon>
            </v-list-tile-action>
            <v-list-tile-content>
              <v-list-tile-title>File upload</v-list-tile-title>
            </v-list-tile-content>
          </v-list-tile>

          <!-- <v-list-tile @click="upload_folder">
            <v-list-tile-action>
              <v-icon>mdi-folder-upload</v-icon>
            </v-list-tile-action>
            <v-list-tile-content>
              <v-list-tile-title>Folder upload</v-list-tile-title>
            </v-list-tile-content>
          </v-list-tile> -->

    </v-list>
    </v-menu>


      <v-tooltip bottom>
        <v-btn slot="activator" icon :disabled="selected.length === 0" @click="move">
          <v-icon color="grey darken-1">mdi-folder-move</v-icon>
        </v-btn>
        <span>Move to...</span>
      </v-tooltip>

      <v-tooltip bottom>
        <v-btn slot="activator" icon :disabled="selected.length === 0" @click="remove">
          <v-icon color="grey darken-1">mdi-delete</v-icon>
        </v-btn>
        <span>Remove</span>
      </v-tooltip>


      <v-tooltip bottom>
        <v-btn slot="activator" icon :disabled="!can_download" @click="download">
          <v-icon color="grey darken-1">mdi-download</v-icon>
        </v-btn>
        <span>Download</span>
      </v-tooltip>
    </v-toolbar-items>
  </v-toolbar>

  <!-- <v-layout v-if="is_loading" row wrap justify-center class="text-xs-center">
    <v-flex xs12>
      <v-progress-circular
      :size="50"
      color="primary"
      indeterminate></v-progress-circular>
    </v-flex>
  </v-layout> -->

  <v-data-table
  :headers="headers"
  :items="files"
  item-key="id"
  hide-actions
  :loading="is_loading"
  v-model="selected">
  <template slot="no-data">
      <p class="text-xs-center">
        Folder is empty.
      </p>
    </template>
  <template slot="items" slot-scope="props">
    <tr class="pointer"
    :active="props.selected"
    @click="props.selected = !props.selected"
    v-on:dblclick="goto_folder(props.item)">
      <td class="text-xs-left">
      <v-layout align-center row fill-height>
        <v-icon v-if="props.item.type === 'application/vnd.bloom.folder'">mdi-folder</v-icon>
        <v-icon v-else>mdi-file</v-icon>
        <span>{{ props.item.name }}</span>
      </v-layout>
    </td>
    <td class="text-xs-left">{{ props.item.updated_at | date }}</td>
    <td class="text-xs-left">
      <span v-if="props.item.size !== 0">{{ props.item.size | filesize }}</span>
      <span v-else>-</span>
    </td>
  </tr>
</template>
</v-data-table>

<input type="file" class="upload-files" ref="upload-files"
  multiple v-on:change="handle_files_upload"/>

<input type="file" class="upload-folder" ref="upload-folder"
  webkitdirectory mozdirectory v-on:change="handle_folder_upload"/>

<blm-drive-dialog-create-folder
:parent="current_folder"
:visible="display_create_folder_dialog"
@close="display_create_folder_dialog = false"
@create="folder_created" />

<blm-drive-dialog-move
:visible="move_dialog"
@close="move_dialog = false"
:selected="selected"
@move="moved" />

<v-dialog
v-model="uploading_dialog"
max-width="400px"
persistent
scrollable
>
<v-card>
  <v-card-title class="headline">
    Uploading {{ files_to_upload.length }}
    item{{ files_to_upload.length === 1 ? '' : 's' }}</v-card-title>
    <v-divider></v-divider>

    <v-card-text style="height: 300px;">
      <v-list >
       <v-list-tile
         v-for="(file, index) in files_to_upload"
         :key="file.name"
       >
         <v-list-tile-content>
           <v-list-tile-title v-html="file.name"></v-list-tile-title>
         </v-list-tile-content>

         <v-list-tile-action>

          <v-icon v-if="files_to_upload[index].state === 'uploaded'" color="green">
            mdi-check-circle
          </v-icon>
        <v-progress-circular
        v-else-if="files_to_upload[index].state === 'uploading'"
        color="primary"
        :value="files_to_upload[index].uploaded_percent">
      </v-progress-circular>
           <v-progress-circular v-else :value="0"></v-progress-circular>
         </v-list-tile-action>
       </v-list-tile>
     </v-list>
       </v-card-text>
       <v-divider></v-divider>
       <v-card-actions>
         <v-btn color="blue darken-1" flat @click.native="cancel_upload">Cancel</v-btn>
       </v-card-actions>

</v-card>
</v-dialog>

  <v-btn
  color="red"
  dark
  fab
  fixed
  bottom
  right
  v-if="fab"
  @click="bottom_sheet = true"
  >
  <v-icon>mdi-plus</v-icon>
</v-btn>

<v-bottom-sheet v-model="bottom_sheet">


<v-list>
  <v-list-tile @click="open_new_folder_dialog">
    <v-list-tile-avatar>
      <v-avatar size="32px" tile>
        <v-icon>mdi-folder-plus</v-icon>
      </v-avatar>
    </v-list-tile-avatar>
    <v-list-tile-title>New Folder</v-list-tile-title>
  </v-list-tile>

  <v-divider></v-divider>

  <v-list-tile @click="upload_files">
    <v-list-tile-action>
      <v-icon>mdi-file-upload</v-icon>
    </v-list-tile-action>
    <v-list-tile-content>
      <v-list-tile-title>File upload</v-list-tile-title>
    </v-list-tile-content>
  </v-list-tile>

  <!-- <v-list-tile @click="upload_folder">
    <v-list-tile-action>
      <v-icon>mdi-folder-upload</v-icon>
    </v-list-tile-action>
    <v-list-tile-content>
      <v-list-tile-title>Folder upload</v-list-tile-title>
    </v-list-tile-content>
  </v-list-tile> -->
</v-list>

</v-bottom-sheet>

</div>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import axios from 'axios';
const { log } = require('@bloom42/astro');
import router from '@/bloom/kernel/router';
import api from '@/bloom/kernel/api';
import util from '@/bloom/kernel/util';
import CreateFolderDialog from './CreateFolderDialog.vue';
import MoveDialog from './MoveDialog.vue';


@Component({
  components: {
    'blm-drive-dialog-create-folder': CreateFolderDialog,
    'blm-drive-dialog-move': MoveDialog,
  },
})
export default class Folder extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  files: any[] = [];
  files_to_upload: any[] = [];
  uploading_dialog = false;
  selected: any[] = [];
  current_folder: any = null;
  axios_cancel_source: any = null;
  bottom_sheet = false;
  fab = false;
  display_create_folder_dialog = false;
  move_dialog = false;
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Name',
      value: 'name',
      width: '50%',
    },
    { text: 'Last modified', value: 'updated_at', sortable: true },
    { text: 'Size', value: 'size', sortable: true },
  ];


  // computed
  get can_download() {
    if (this.selected.length === 1) {
      if (this.selected[0].type !== 'folder') {
        return true;
      }
    }
    return false;
  }

  get search_query() {
    return this.$store.state.search;
  }


  // lifecycle
  async created() {
    if (!this.$store.state.drive_profile) {
      await this.fetch_profile();
    }
    this.fetch_data(this.$route.params.folder_id);
    window.addEventListener('resize', this.check_window_size);
    if (this.$vuetify.breakpoint.smAndDown) {
      this.fab = true;
    } else {
      this.fab = false;
    }
  }

  destroyed() {
    window.removeEventListener('resize', this.check_window_size);
  }


  // watch
  @Watch('$route')
  on_route_change(to: any) {
    this.fetch_data(to.params.folder_id);
  }

  @Watch('search_query')
  on_search_query_change() {
    if (this.search_query) {
      this.search(this.search_query);
    } else {
      this.fetch_data(this.$route.params.folder_id);
    }
  }


  // methods
  open_new_folder_dialog() {
    this.bottom_sheet = false;
    this.display_create_folder_dialog = true;
  }

  check_window_size() {
    if (this.$vuetify.breakpoint.smAndDown) {
      this.fab = true;
    } else {
      this.fab = false;
    }
  }

  goto_folder(file: any) {
    if (file.type === 'application/vnd.bloom.folder') {
      this.selected = [];
      router.push({ path: `/drive/folders/${file.id}` });
    }
  }

  async fetch_profile() {
    this.error = '';
    this.is_loading = true;
    try {
      const res = await api.get(`${api.DRIVE}/v1/me`);
      this.$store.commit('set_drive_profile', res);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async search(_query: string) {
    this.error = '';
    try {
      this.is_loading = true;
      // const data = await api.query(
      //   api.DRIVE_GRAPHQL,
      //   `query($query: String!) {
      //     search(query: $query) {
      //       id
      //       created_at
      //       updated_at
      //       name
      //       type
      //       size
      //     }
      //   }
      //   `,
      //   { query },
      // );
      // this.files = data.search;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async fetch_data(id: string) {
    this.error = '';
    this.is_loading = true;
    try {
      const folder = await api.get(`${api.DRIVE}/v1/folders`, {
        params: { id },
      });
      this.files = folder.files;
      folder.clean_path = folder.path.map((path_elem: any, i: number) => {
        const isRoot = path_elem.name === '__BLOOM_ROOT' && i === 0;
        const text = isRoot ? 'My Drive' : path_elem.name;
        const to = isRoot ? '/drive' : `/drive/folders/${path_elem.id}`;
        return {
          disabled: false,
          exact: true,
          text,
          to,
        };
      });
      this.current_folder = folder;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  upload_files() {
    this.bottom_sheet = false;
    (this.$refs['upload-files'] as HTMLElement).click();
  }
  handle_files_upload() {
    const uploadedFiles = (this.$refs['upload-files'] as any).files;

    Array.from(uploadedFiles).forEach((_: any, i: number) => {
      uploadedFiles[i].state = 'pending';
      this.files_to_upload.push(uploadedFiles[i]);
    });
    if (this.files_to_upload.length === 0) {
      return;
    }

    this.submit_files();
  }

  upload_folder() {
    this.bottom_sheet = false;
    (this.$refs['upload-folder'] as HTMLElement).click();
  }

  async handle_folder_upload() {
    const files_to_upload = (this.$refs['upload-folder'] as any).files;

    if (files_to_upload.length === 0) {
      return;
    }
    this.uploading_dialog = true;
    const file_uploading = { name: 'Folder', state: 'uploading', uploaded_percent: 0 };
    this.files_to_upload = [file_uploading];
    try {
      const client = api.client();
      const total_files_to_upload = files_to_upload.length;
      let n_uploaded = 0;
      Array.from(files_to_upload).forEach(async (file: any) => {
        this.axios_cancel_source = axios.CancelToken.source();
        const formData = new FormData();
        formData.append('file', file);
        formData.append('parent', this.current_folder.id);
        const res = await client.post(api.DRIVE_UPLOAD,  // eslint-disable-line
          formData,
          {
            cancelToken: this.axios_cancel_source.token,
            headers: { 'Content-Type': 'multipart/form-data' },
          });
        const profile = this.$store.state.drive_profile;
        profile.used_space += res.data.data.size;
        this.$store.commit('set_drive_profile', profile);
        n_uploaded += 1;
        file_uploading.uploaded_percent = Math.ceil((n_uploaded / total_files_to_upload) * 100);
        this.$set(this.files_to_upload, 0, file_uploading);
      });
      await this.fetch_data(this.current_folder.id);
      this.$toast.success('Success', { icon: 'mdi-check-circle' });
    } catch (err) {
      if (axios.isCancel(err)) {
        log.debug('upload canceled');
        return;
      }
      this.error = err.message;
    } finally {
      this.uploading_dialog = false;
      this.files_to_upload = [];
      this.axios_cancel_source = null;
    }
  }

  folder_created(folder: any) {
    this.files.push(folder);
  }

  async submit_files() {
    this.uploading_dialog = true;

    try {
      // const client = api.client();
      for (let i = 0; i < this.files_to_upload.length; i += 1) {
        const file = this.files_to_upload[i];
        file.state = 'uploading';
        file.uploaded_percent = 0;
        this.$set(this.files_to_upload, i, file);

        // start upload
        const start_payload = {
          file_name: file.name,
          parent_id: this.current_folder.id,
        };
        const res = await api.post(`${api.DRIVE}/v1/uploads`, start_payload);
        const upload_id = res.id;

        // upload options
        this.axios_cancel_source = axios.CancelToken.source();
        const options = {
          cancelToken: this.axios_cancel_source.token,
          headers: {
            'Content-Type': 'multipart/form-data',
          },
          onUploadProgress: (progressEvent: ProgressEvent) => {
            file.uploaded_percent = Math.ceil(
              (progressEvent.loaded / progressEvent.total) * 100,
            );
            this.$set(this.files_to_upload, i, file);
          },
          // transformRequest: [(data: any, headers: any) => {
          //   delete headers.common.Authorization;
          //   return data;
          // }],
        };
        const formData = new FormData();
        formData.append('file', file);

        // complete upload
        const new_file = await api.put(`${api.DRIVE}/v1/uploads/${upload_id}`, formData, options);

        this.files.push(new_file);
        const profile = Object.assign({}, this.$store.state.drive_profile);
        profile.used_space += new_file.size;
        this.$store.commit('set_drive_profile', profile);
        file.state = 'uploaded';
        this.$set(this.files_to_upload, i, file);

        // const formData = new FormData();
        // formData.append('file', this.files_to_upload[i]);
        // formData.append('parent', this.current_folder.id);
        // const res = await client.post(api.DRIVE_UPLOAD,  // eslint-disable-line
        //   formData,
        //   {
        //
        //     headers: { 'Content-Type': 'multipart/form-data' },
        //
        //   });
      }

      this.$toast.success('Success', { icon: 'mdi-check-circle' });
    } catch (err) {
      if (axios.isCancel(err)) {
        log.debug('upload canceled');
        return;
      }
      this.error = err.message;
    } finally {
      this.uploading_dialog = false;
      this.files_to_upload = [];
      this.axios_cancel_source = null;
    }
  }

  cancel_upload() {
    this.uploading_dialog = false;
    if (this.axios_cancel_source) {
      this.axios_cancel_source.cancel();
    }
  }

  async remove() {
    this.error = '';
    const files = this.selected.map((f: any) => f.id);
    const payload = {
      files,
    };
    try {
      this.is_loading = true;
      await api.post(`${api.DRIVE}/v1/trash`, payload);
      this.files = this.files.filter((f: any) => files.indexOf(f.id) === -1);
      this.selected = [];
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async download() {
    if (this.selected.length !== 1) {
      return;
    }
    const { id } = this.selected[0];
    this.error = '';
    this.is_loading = true;
    try {
      const res = await api.get(`${api.DRIVE}/v1/files/${id}/url`);
      util.download_file(res.url);
      this.selected = [];
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  move() {
    this.move_dialog = true;
  }

  moved(to: string) {
    if (to !== this.current_folder!.id) {
      const files = this.selected.map((f: any) => f.id);
      this.files = this.files.filter((f: any) => files.indexOf(f.id) === -1);
    }
    this.selected = [];
  }
}
</script>


<style scoped lang="scss">
td span {
  padding-left: 5px;
}

.new-btn {
  height: 36px !important;
}

.upload-files, .upload-folder {
  display: none;
}
</style>
