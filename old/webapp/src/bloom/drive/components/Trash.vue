<template>
  <div>
    <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
      {{ error }}
    </v-alert>

  <v-toolbar flat dense>
    <v-spacer></v-spacer>
    <v-toolbar-items>

      <v-tooltip bottom>
        <v-btn slot="activator" icon :disabled="selected.length === 0" @click="restore">
          <v-icon color="grey darken-1">mdi-restore-clock</v-icon>
        </v-btn>
        <span>Restore</span>
      </v-tooltip>

      <v-tooltip bottom>
        <v-btn slot="activator" icon :disabled="selected.length === 0" @click="delete_forever">
          <v-icon color="grey darken-1">mdi-delete</v-icon>
        </v-btn>
        <span>Delete forever</span>
      </v-tooltip>

    </v-toolbar-items>
  </v-toolbar>

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
    @click="props.selected = !props.selected">
      <td class="text-xs-left">
      <v-layout align-center row fill-height>
        <v-icon v-if="props.item.type === 'application/vnd.bloom.folder'">mdi-folder</v-icon>
        <v-icon v-else>mdi-file</v-icon>
        <span>{{ props.item.name }}</span>
      </v-layout>
    </td>
    <td class="text-xs-left">{{ props.item.updated_at | calendar }}</td>
    <td class="text-xs-left">
      <span v-if="props.item.size !== 0">{{ props.item.size | filesize }}</span>
      <span v-else>-</span>
    </td>
  </tr>
</template>
</v-data-table>
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class Trash extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  files = [];
  selected = [];
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
  // lifecycle
  async created() {
    if (!this.$store.state.drive_profile) {
      await this.fetch_profile();
    }
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    try {
      this.files = await api.get(`${api.DRIVE}/v1/trash`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async restore() {
    this.error = '';
    const files = this.selected.map((f: any) => f.id);
    const payload = {
      files,
    };
    this.is_loading = true;
    try {
      await api.post(`${api.DRIVE}/v1/files/restore`, payload);
      this.files = this.files.filter((f: any) => files.indexOf(f.id) === -1);
      this.selected = [];
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async delete_forever() {
    this.error = '';
    const files = this.selected.map((f: any) => f.id);
    const payload = {
      files,
    };
    this.is_loading = true;
    try {
      const res = await api.post(`${api.DRIVE}/v1/files/delete`, payload);
      const deleted = this.files.filter((f: any) => files.indexOf(f.id) !== -1);
      this.files = this.files.filter((f: any) => files.indexOf(f.id) === -1);
      const profile = Object.assign({}, this.$store.state.drive_profile);
      profile.used_space -= res.space_freed;
      this.$store.commit('set_drive_profile', profile);
      this.selected = [];
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
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
}
</script>


<style scoped lang="scss">
td span {
  padding-left: 5px;
}

.new-btn {
  height: 36px !important;
}

.upload-files {
  display: none;
}
</style>
