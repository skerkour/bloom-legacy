<template>
  <v-container fluid>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar flat dense>
      <v-spacer />
      <v-btn color="primary" @click="createPlaylist">
        <v-icon left>mdi-plus</v-icon>New Playlist
      </v-btn>
    </v-toolbar>

    <v-container fluid>
      <v-row dense>
        <v-col cols="2" sm="4" lg="3" v-for="playlist in playlists" :key="playlist.id">
          <router-link :to="`/music/playlists/${playlist.id}`" slot="activator">
            <v-img src="/assets/imgs/music_playlist.svg" height="150px" contain class="app-icon"/>
            <p class="headline text-center app-name mt-3">{{ playlist.name }}</p>
          </router-link>
        </v-col>
      </v-row>

    </v-container>


    <blm-music-dialog-new-playlist
      :visible="isNewPlaylistDialogVisible"
      @closed="closeNewPlaylistDialog"
    />
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import NewPlaylistDialog from '../components/new_playlist_dialog.vue';


@Component({
  components: {
    'blm-music-dialog-new-playlist': NewPlaylistDialog,
  },
})
export default class Playlists extends Vue {
  // props
  // data
  error = '';
  playlists = [
    { name: 'Hello world', id: '1' },
  ];
  isNewPlaylistDialogVisible = false;


  // computed
  // lifecycle
  // watch
  // methods
  createPlaylist() {
    this.isNewPlaylistDialogVisible = true;
  }

  closeNewPlaylistDialog() {
    this.isNewPlaylistDialogVisible = false;
  }
}
</script>


<style lang="scss" scoped>
.app-icon {
  border-radius: 28px;
}

.v-application a {
  color: inherit !important;
}
</style>
