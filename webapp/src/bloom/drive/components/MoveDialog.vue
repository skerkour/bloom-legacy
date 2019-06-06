<template>
  <v-dialog v-model="show" max-width="400px" @keydown.esc="close" scrollable>
    <v-card>
    <v-card-title class="indigo white--text headline">
      User Directory
    </v-card-title>

    <v-card-text style="height: 400px;">
      <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
        {{ error }}
      </v-alert>
    <v-layout
      justify-space-between
      pa-3
    >
      <v-flex>
        <v-treeview
          :active.sync="to"
          :items="items"
          item-key="id"
          :load-children="fetch_folder"
          :open.sync="open"
          activatable
          class="grey lighten-5"
          indeterminate-icon="mdi-loading"
          item-children="files"
        >
          <!-- <v-icon
            v-if="!item.files"
            slot="prepend"
            slot-scope="{ item, active }"
            :color="active ? 'primary' : ''"
          >mdi-folder</v-icon> -->
        </v-treeview>
      </v-flex>
    </v-layout>
  </v-card-text>

  <v-card-actions>
    <v-spacer></v-spacer>
    <v-btn flat @click="close" :disabled="is_loading">Cancel</v-btn>
    <v-btn
      @click="move"
      color="primary"
      :loading="is_loading"
      :disabled="!can_move">
    Move
  </v-btn>
</v-card-actions>
  </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component({})
export default class MoveDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: Array, default: [] }) selected!: any[];


  // data
  error = '';
  is_loading = false;
  to = [];
  open = [];
  items2: any[] = [];

  // computed
  get can_move() {
    if (this.to.length === 1 && this.to) {
      return true;
    }
    return false;
  }

  get items() {
    if (this.items2.length === 0) {
      this.items2 = [{
        files: [],
        id: this.$store.state.drive_profile.home,
        name: 'My Drive',
      }];
    }
    return this.items2;
  }

  set items(items: any) {
    this.items2 = items;
  }

  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('close');
    }
  }


  // lifecycle
  // watch
  // methods
  async fetch_folder(file: any) {
    this.error = '';
    this.is_loading = true;
    const id = file ? file.id : undefined;
    try {
      const folder = await api.get(`${api.DRIVE}/v1/folders`, {
        params: { id },
      });
      const files = folder.files
        .filter((child: any) => child.type === 'application/vnd.bloom.folder')
        .map((child: any) => {
          child.files = [];
          return child;
        });
      if (file) {
        file.files.push(...files);
      }
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async move() {
    this.error = '';
    if (!this.can_move) {
      return;
    }
    this.is_loading = true;
    const files = this.selected.map((f: any) => f.id);
    try {
      const to = this.to[0];
      const payload = {
        files,
        to,
      };
      await api.post(`${api.DRIVE}/v1/files/move`, payload);
      this.$emit('move', to);
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  close() {
    this.show = false;
    this.to = [];
    this.$emit('close');
  }
}
</script>
