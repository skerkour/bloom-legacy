<template>
<div>
  <v-container >
    <v-layout row>
      <v-flex xs12>
        <div class="elevation-0">
          <v-container fluid grid-list-md>
            <v-layout row wrap justify-left>
              <v-flex xs4 sm3 lg2 justify-center class="pointer" @click="open_new_album_dialog">
                  <v-img src="/kernel/static/imgs/plus.svg" height="150px" contain class="app-icon">
                  </v-img>
                  <p class="headline text-xs-center app-name mt-3">Create album</p>
              </v-flex>
              <v-flex v-for="album in albums" xs4 sm3 lg2 :key="album.id">
                <router-link :to="`/gallery/albums/${album.id}`" slot="activator">
                  <v-img src="/gallery/static/imgs/album.svg" height="150px" contain class="app-icon"></v-img>
                  <p class="headline text-xs-center app-name mt-3">{{ album.name }}</p>
                </router-link>
              </v-flex>
            </v-layout>
          </v-container>
        </div>
      </v-flex>
    </v-layout>
  </v-container>


  <blm-gallery-dialog-create-album
  :visible="display_create_album_dialog"
  @close="display_create_album_dialog = false"
  @create="album_created" />
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import CreateAlbumDialog from './CreateAlbumDialog.vue';


@Component({
  components: {
    'blm-gallery-dialog-create-album': CreateAlbumDialog,
  },
})
export default class Index extends Vue {
  // props
  // data
  albums: any[] = [];
  is_loading = false;
  error = '';
  display_create_album_dialog = false;


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
      this.albums = await api.get(`${api.GALLERY}/v1/albums`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  album_created(album: any) {
    this.albums.push(album);
  }

  open_new_album_dialog() {
    this.display_create_album_dialog = true;
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
