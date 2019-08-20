<template>
  <v-dialog
  v-model="show"
  max-width="400px"
  @keydown.esc="show = false"
  >
  <v-card>
    <v-card-title class="blue darken-1 white--text headline">
      Rename {{ file.name }}
    </v-card-title>

    <v-card-text>
      <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
        {{ error }}
      </v-alert>
      <v-text-field
      label="New name"
      outline
      v-model="new_file_name"
      :disabled="is_loading"
      @keyup.enter.native="rename_file"
      ></v-text-field>
    </v-card-text>

    <v-card-actions>
      <v-spacer></v-spacer>
      <v-btn flat @click="show = false" :disabled="is_loading">Cancel</v-btn>
      <v-btn
      @click="rename_file"
      color="primary"
      :loading="is_loading"
      >
      Rename
    </v-btn>
  </v-card-actions>
</v-card>
</v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class CreateFolderDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: Object, default: null }) file!: any;


  // data
  error = '';
  is_loading = false;
  new_file_name = '';


  // computed
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
  @Watch('file')
  on_file_changed() {
    if (this.file) {
      this.new_file_name = this.file.name;
    } else {
      this.new_file_name = '';
    }
  }


  // methods
  async rename_file() {
    this.error = '';
    this.is_loading = true;
    const payload = {
      name: this.new_file_name,
    };
    try {
      const res = await api.put(`${api.DRIVE}/v1/files/${this.file.id}`, payload);
      this.$emit('rename', res);
      this.new_file_name = '';
      this.show = false;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>
