<template>
  <v-dialog
  v-model="show"
  max-width="75vw"
  @keydown.esc="show = false"
  >
  <v-card>
    <v-card-title class="blue darken-1 white--text headline">
      Add media to {{ album.name }}
    </v-card-title>

    <v-card-text class="text-xs-center">
      <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
        {{ error }}
      </v-alert>
      <v-progress-circular
        v-if="is_loading"
        indeterminate
        color="primary"
        :size="84"
      ></v-progress-circular>
      <v-card class="elevation-0">
        <v-container grid-list-sm fluid>
          <v-layout row wrap>
            <v-flex v-for="file in media" :key="file.id" xs4 sm3 md2 d-flex>
              <v-card flat tile @click="select_file(file)"
    :class="{pointer: true, 'd-flex': true, selected: file.selected, 'elevation-2': file.selected}">
                <v-img :src="file.url" class="grey lighten-4">
                  <template v-slot:placeholder>
                    <v-layout fill-height align-center justify-center ma-0>
                      <v-progress-circular indeterminate color="grey lighten-5">
                      </v-progress-circular>
                    </v-layout>
                  </template>
                </v-img>
              </v-card>
            </v-flex>
          </v-layout>
        </v-container>
      </v-card>
    </v-card-text>

    <v-card-actions>
      <v-spacer></v-spacer>
      <v-btn flat @click="show = false" :disabled="is_loading">Cancel</v-btn>
      <v-btn
        color="primary"
        @click="add_media"
        :loading="is_loading"
        :disabled="!can_add">
        Add
      </v-btn>
    </v-card-actions>
  </v-card>
</v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class CreateAlbumDialog extends Vue {
  // props
  @Prop({ type: Object, default: {} }) album!: any;
  @Prop({ type: Array, default: [] }) media_already_in!: any[];
  @Prop({ type: Boolean, default: false }) visible!: boolean;


  // data
  error = '';
  is_loading = false;
  media: any[] = [];
  selected: any = {};


  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('close');
      this.selected = {};
    }
  }

  get can_add() {
    if (Object.keys(this.selected).length > 0) {
      return true;
    }
    return false;
  }


  // lifecycle
  // watch
  @Watch('visible')
  on_visible_change(to: any) {
    if (to === true) {
      this.fetch_data();
    }
  }


  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    const already_in = this.media_already_in.reduce((acc: any, m: any) => {
      acc[m.id] = true;
      return acc;
    }, {});
    try {
      const media = await api.get(`${api.GALLERY}/v1/media`);
      this.media = media.filter((m: any) => !already_in[m.id]).map((m: any) => {
        m.selected = false;
        return m;
      });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  select_file(file: any) {
    file.selected = !file.selected;
    if (file.selected) {
      this.$set(this.selected, file.id, true);
      // this.selected[file.id] = true;
    } else if (!file.selected && file.id in this.selected) {
      this.$delete(this.selected, file.id);
    }
  }

  async add_media() {
    this.error = '';
    this.is_loading = true;
    const payload = {
      media: Object.keys(this.selected),
    };
    try {
      await api.post(`${api.GALLERY}/v1/albums/${this.album.id}/add`, payload);
      this.$emit('add');
      this.show = false;
      this.selected = {};
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
.selected {
  border-style: solid;
  border-width: 2px;
  border-color: #409EFF;
  border-radius: 4px;
}
</style>
