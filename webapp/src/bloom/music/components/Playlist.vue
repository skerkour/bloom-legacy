<template>
<div>
  <v-layout row wrap>
    <v-flex v-if="playlist" xs12 justify-center class="text-xs-center">
      <v-toolbar flat dense>
        <v-toolbar-title>{{ playlist.name }}</v-toolbar-title>
        <v-spacer></v-spacer>
        <v-toolbar-items>

          <v-tooltip bottom>
            <v-btn
              flat color="primary" class="hidden-sm-and-down" slot="activator"
              @click="open_add_dialog">
              <v-icon left>mdi-plus</v-icon>Add
            </v-btn>
            <span>Add songs to Playlist</span>
          </v-tooltip>

          <v-tooltip bottom>
            <v-btn
              flat color="primary" class="hidden-sm-and-down" slot="activator"
              @click="open_remove_dialog">
              <v-icon left>mdi-minus</v-icon>Remove
            </v-btn>
            <span>Remove songs from Playlist</span>
          </v-tooltip>

          <v-tooltip bottom>
            <v-btn slot="activator" icon @click="open_rename_dialog">
              <v-icon color="grey darken-1">mdi-alphabetical</v-icon>
            </v-btn>
            <span>Rename Playlist</span>
          </v-tooltip>

          <v-tooltip bottom>
            <v-btn slot="activator" icon @click="delete_playlist" :loading="is_loading">
              <v-icon color="grey darken-1">mdi-delete</v-icon>
            </v-btn>
            <span>Delete Playlist</span>
          </v-tooltip>

        </v-toolbar-items>
      </v-toolbar>
    </v-flex>


    <v-flex xs12>
      <v-card class="elevation-0 tracks-table">

          <v-data-table
            :headers="headers"
            :items="musics"
            :loading="is_loading"
            hide-actions
          >
            <template slot="no-data">
              <p class="text-xs-center">
                No Music in this playlist.
              </p>
            </template>
            <v-progress-linear v-slot:progress color="blue" indeterminate></v-progress-linear>
            <template v-slot:items="props">
              <tr class="pointer"
                :active="props.index === index"
                @click="props.selected = !props.selected"
                v-on:dblclick="play_music(props.index)">
                  <td class="text-xs-left">
                  <v-layout align-center row fill-height>
                    <span>{{ props.item.name }}</span>
                  </v-layout>
                </td>
              </tr>
            </template>
          </v-data-table>

      </v-card>
    </v-flex>
  </v-layout>

  <blm-music-player id="music-player" :musics="musics" :index="index" @update="play_music"/>

  <blm-music-dialog-rename-playlist
    :playlist="playlist"
    :visible="display_rename_playlist_dialog"
    @close="display_rename_playlist_dialog = false"
    @rename="playlist_renamed" />

  <blm-music-dialog-add-songs-to-playlist
    :playlist="playlist"
    :visible="display_add_songs_to_playlist_dialog"
    :songs_already_in="musics"
    @close="display_add_songs_to_playlist_dialog = false"
    @add="added_to_playlist" />

  <blm-music-dialog-remove-songs-from-playlist
    :playlist="playlist"
    :visible="display_remove_dialog"
    :songs_already_in="musics"
    @close="display_remove_dialog = false"
    @remove="songs_removed" />
</div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import router from '@/bloom/kernel/router';
import DialogAddSongsToPlaylist from './DialogAddSongsToPlaylist.vue';
import DialogRenamePlaylist from './DialogRenamePlaylist.vue';
import DialogRemoveSongsFromPlaylist from './DialogRemoveSongsFromPlaylist.vue';
import Player from './Player.vue';


@Component({
  components: {
    'blm-music-dialog-add-songs-to-playlist': DialogAddSongsToPlaylist,
    'blm-music-dialog-remove-songs-from-playlist': DialogRemoveSongsFromPlaylist,
    'blm-music-dialog-rename-playlist': DialogRenamePlaylist,
    'blm-music-player': Player,
  },
})
export default class Index extends Vue {
  // props
  // data
  musics: any[] = [];
  playlist: any = null;
  is_loading = false;
  display_rename_playlist_dialog = false;
  display_add_songs_to_playlist_dialog = false;
  display_remove_dialog = false;
  error = '';
  index = -1;
  headers = [
    {
      align: 'left',
      sortable: false,
      text: 'Track',
      value: 'name',
    },
  ];


  // computed
  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    try {
      const res = await api.get(`${api.MUSIC}/v1/playlists/${this.$route.params.playlist_id}`);
      this.musics = res.musics;
      this.playlist = res.playlist;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async delete_playlist() {
    if (this.playlist === null) {
      return;
    }
    this.error = '';
    this.is_loading = true;
    try {
      await api.delete(`${api.MUSIC}/v1/playlists/${this.$route.params.playlist_id}`);
      router.push({ path: '/music/playlists' });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  play_music(index: number) {
    this.index = index;
  }

  playlist_renamed(playlist: any) {
    this.playlist = playlist;
  }
  added_to_playlist() {
    this.fetch_data();
  }
  songs_removed() {
    this.fetch_data();
  }

  open_rename_dialog() {
    this.display_rename_playlist_dialog = true;
  }
  open_add_dialog() {
    this.display_add_songs_to_playlist_dialog = true;
  }
  open_remove_dialog() {
    this.display_remove_dialog = true;
  }
}
</script>


<style scoped lang="scss">
.add-btn {
  height: 36px !important;
}

#music-player {
  position: fixed;
  bottom: 0px;
  width: calc(100% - 250px);

  @media screen and (max-width: 1264px) {
    width: 100%;
  }
}

.tracks-table {
  margin-bottom: 125px;
}
</style>
