<template>
  <v-dialog
  v-model="show"
  max-width="400px"
  @keydown.esc="show = false"
  >
  <v-card>
    <v-card-title class="blue darken-1 white--text headline">
      Rename {{ album.name }}
    </v-card-title>

    <v-card-text>
      <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
        {{ error }}
      </v-alert>
      <v-text-field
      label="New Name"
      outline
      v-model="new_album_name"
      :disabled="is_loading"
      @keyup.enter.native="rename_album"
      ></v-text-field>
    </v-card-text>

    <v-card-actions>
      <v-spacer></v-spacer>
      <v-btn flat @click="show = false" :disabled="is_loading">Cancel</v-btn>
      <v-btn
      @click="rename_album"
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
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class CreateAlbumDialog extends Vue {
  // props
  @Prop({ type: Object, default: {} }) album!: any;
  @Prop({ type: Boolean, default: false }) visible!: boolean;


  // data
  error = '';
  is_loading = false;
  new_album_name = '';


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
  // methods
  async rename_album() {
    this.error = '';
    this.is_loading = true;
    const payload = {
      name: this.new_album_name,
    };
    try {
      const res = await api.put(`${api.GALLERY}/v1/albums/${this.album.id}`, payload);
      this.$emit('rename', res);
      this.new_album_name = '';
      this.show = false;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>
