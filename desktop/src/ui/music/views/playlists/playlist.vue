<template>
  <v-container fluid>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar flat dense>
      <v-toolbar-title>{{ playlist.name }} </v-toolbar-title>
      <v-spacer />

      <v-btn color="primary" @click="addSongs">
        <v-icon left>mdi-plus</v-icon>Add songs
      </v-btn>

      <v-btn color="red" text class="mx-2" @click="removeSongs">
        <v-icon left>mdi-minus</v-icon>Remove songs
      </v-btn>

      <v-menu>
        <template v-slot:activator="{ on: menu }">
          <v-tooltip bottom>
            <template v-slot:activator="{ on: tooltip }">
              <v-btn icon v-on="{ ...tooltip, ...menu }">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
            <span>More</span>
          </v-tooltip>
        </template>

        <v-list>
          <v-list-item @click="renamePlaylist">
            <v-list-item-icon>
              <v-icon>mdi-alphabetical</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Rename Playlist</v-list-item-title>
          </v-list-item>
          <v-list-item @click="deletePlaylist">
            <v-list-item-icon>
              <v-icon>mdi-delete</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Delete Playlist</v-list-item-title>
          </v-list-item>
      </v-list>

      </v-menu>
    </v-toolbar>

    <v-data-table
      :headers="headers"
      :items="songs"
      item-key="id"
      :loading="isLoading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No songs.
        </p>
      </template>

      <template v-slot:item="{ item }" class="text-left">
        <td>
          <span>{{ item.name | truncate }}</span>
        </td>
        <td>
          <span>{{ item.artist | truncate }}</span>
        </td>
        <td>
          <span>{{ item.album | truncate }}</span>
        </td>
        <td>
          <span>{{ item.duration }}</span>
        </td>
      </template>
    </v-data-table>

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Playlists extends Vue {
  // props
  // data
  error = '';
  songs = [];
  isLoading = false;
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Name',
      value: 'name',
      width: '40%',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Artist',
      value: 'artist',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Album',
      value: 'album',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Time',
      value: 'duration',
    },
  ];
  playlist = {
    name: 'Hello World',
  };
  // computed
  // lifecycle
  // watch
  // methods
  renamePlaylist() {
  }

  deletePlaylist() {
  }

  addSongs() {
  }

  removeSongs() {
  }
}
</script>


<style lang="scss" scoped>
.app-icon {
  border-radius: 28px;
}
</style>
