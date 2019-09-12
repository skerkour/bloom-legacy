<template>
  <v-dialog v-model="show" max-width="75vw" @keydown.esc="show = false">
  <v-card>
    <v-card-title class="blue darken-1 white--text headline">
      Add songs to {{ playlist.name }}
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
          <v-list>
            <v-list-tile  v-for="file in songs" :key="file.id" @click="select_file(file)"
    :class="{pointer: true, 'd-flex': true, selected: file.selected, 'elevation-2': file.selected}">
              <v-list-tile-content>
                <v-list-tile-title>{{ file.name }}</v-list-tile-title>
              </v-list-tile-content>
            </v-list-tile>
          </v-list>
        </v-container>
      </v-card>
    </v-card-text>

    <v-card-actions>
      <v-spacer></v-spacer>
      <v-btn flat @click="show = false" :disabled="is_loading">Cancel</v-btn>
      <v-btn
        color="primary"
        @click="add_songs"
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
export default class DialogAddSongsToPlaylist extends Vue {
  // props
  @Prop({ type: Object, default: {} }) playlist!: any;
  @Prop({ type: Array, default: [] }) songs_already_in!: any[];
  @Prop({ type: Boolean, default: false }) visible!: boolean;


  // data
  error = '';
  is_loading = false;
  songs: any[] = [];
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
    const already_in = this.songs_already_in.reduce((acc: any, m: any) => {
      acc[m.id] = true;
      return acc;
    }, {});
    try {
      const songs = await api.get(`${api.MUSIC}/v1/musics`);
      this.songs = songs.filter((m: any) => !already_in[m.id]).map((m: any) => {
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

  async add_songs() {
    this.error = '';
    this.is_loading = true;
    const payload = {
      musics: Object.keys(this.selected),
    };
    try {
      await api.post(`${api.MUSIC}/v1/playlists/${this.playlist.id}/add`, payload);
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
  background-color: #e9e9e9;
}
</style>
