<template>
<div>
  <v-container >
    <v-layout row>
      <v-flex xs12>
        <v-card class="elevation-0">
          <v-container fluid grid-list-md>
            <v-layout row wrap justify-left>
              <v-flex xs4 sm3 lg2 justify-center class="pointer" @click="open_new_playlist_dialog">
                  <v-img src="/kernel/static/imgs/plus.svg" height="150px" contain class="app-icon">
                  </v-img>
                  <p class="headline text-xs-center app-name mt-3">Create playlist</p>
              </v-flex>
              <v-flex v-for="playlist in playlists" xs4 sm3 lg2 :key="playlist.id">
                <router-link :to="`/music/playlists/${playlist.id}`" slot="activator">
                  <v-img src="/music/static/imgs/playlist.svg" height="150px" contain class="app-icon"></v-img>
                  <p class="headline text-xs-center app-name mt-3">{{ playlist.name }}</p>
                </router-link>
              </v-flex>
            </v-layout>
          </v-container>
        </v-card>
      </v-flex>
    </v-layout>
  </v-container>


  <blm-music-dialog-create-playlist
  :visible="display_create_playlist_dialog"
  @close="display_create_playlist_dialog = false"
  @create="playlist_created" />
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import DialogCreatePlaylist from './DialogCreatePlaylist.vue';


@Component({
  components: {
    'blm-music-dialog-create-playlist': DialogCreatePlaylist,
  },
})
export default class Playlists extends Vue {
  // props
  // data
  playlists: any[] = [];
  is_loading = false;
  error = '';
  display_create_playlist_dialog = false;


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
      this.playlists = await api.get(`${api.MUSIC}/v1/playlists`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  playlist_created(playlist: any) {
    this.playlists.push(playlist);
  }

  open_new_playlist_dialog() {
    this.display_create_playlist_dialog = true;
  }
}
</script>


<style scoped lang="scss">
.v-card {
  border-radius: 8px;
}

.v-card__actions p {
  width: 100%;
}

a {
  text-decoration: none;
}

.app-icon {
  border-radius: 28px;
}

.app-name {
  color: #787878;
}
</style>
