<template>
<div>
  <v-layout row wrap>

    <v-flex v-if="album" xs12 justify-center class="text-xs-center">
      <v-toolbar flat dense>
        <v-toolbar-title>{{ album.name }}</v-toolbar-title>
        <v-spacer></v-spacer>
        <v-toolbar-items>

          <v-tooltip bottom>
            <v-btn
              flat color="primary" class="hidden-sm-and-down" slot="activator"
              @click="open_add_dialog">
              <v-icon left>mdi-plus</v-icon>Add
            </v-btn>
            <span>Add media to Album</span>
          </v-tooltip>

          <v-tooltip bottom>
            <v-btn
              flat color="primary" class="hidden-sm-and-down" slot="activator"
              @click="open_remove_dialog">
              <v-icon left>mdi-minus</v-icon>Remove
            </v-btn>
            <span>Remove media from Album</span>
          </v-tooltip>

          <v-tooltip bottom>
            <v-btn slot="activator" icon @click="open_rename_dialog">
              <v-icon color="grey darken-1">mdi-alphabetical</v-icon>
            </v-btn>
            <span>Rename Album</span>
          </v-tooltip>

          <v-tooltip bottom>
            <v-btn slot="activator" icon @click="delete_album" :loading="is_loading">
              <v-icon color="grey darken-1">mdi-delete</v-icon>
            </v-btn>
            <span>Delete Album</span>
          </v-tooltip>

        </v-toolbar-items>
      </v-toolbar>
    </v-flex>


    <v-flex v-if="media.length === 0" xs12>
      <v-container>
        <p>No media in this album</p>
      </v-container>
    </v-flex>

    <v-flex v-if="album === null" xs12 justify-center class="text-xs-center">
      <v-progress-circular
      indeterminate
      color="primary"
      :size="84"
      ></v-progress-circular>
    </v-flex>

    <v-flex xs12 sm9 offset-sm1>
      <div class="elevation-0">
        <v-container grid-list-sm fluid>
          <v-layout row wrap>

            <v-flex v-for="file in media" :key="file.id" xs4 sm3 md2 d-flex
              @click="open_carousel_dialog" class="pointer">
              <v-card flat tile class="d-flex" >
                <v-img :src="file.url" class="grey lighten-2" contain>
                  <template v-slot:placeholder>
                    <v-layout fill-height align-center justify-center ma-0>
                      <v-progress-circular indeterminate color="grey lighten-5"></v-progress-circular>
                    </v-layout>
                  </template>
                </v-img>
              </v-card>
            </v-flex>
          </v-layout>
        </v-container>
      </div>
    </v-flex>
  </v-layout>

  <blm-gallery-dialog-rename-album
    :album="album"
    :visible="display_rename_album_dialog"
    @close="display_rename_album_dialog = false"
    @rename="album_renamed" />

  <blm-gallery-dialog-add-media-to-album
    :album="album"
    :visible="display_add_media_to_album_dialog"
    :media_already_in="media"
    @close="display_add_media_to_album_dialog = false"
    @add="added_to_album" />

  <blm-gallery-dialog-remove-media-from-album
    :album="album"
    :visible="display_remove_dialog"
    :media_already_in="media"
    @close="display_remove_dialog = false"
    @remove="media_removed" />

  <blm-gallery-dialog-carousel
    :media="media"
    :visible="display_carousel_dialog"
    @close="display_carousel_dialog = false" />
</div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import router from '@/bloom/kernel/router';
import RenameAlbumDialog from './RenameAlbumDialog.vue';
import AddMediaToAlbumDialog from './AddMediaToAlbumDialog.vue';
import RemoveMediaFromAlbumDialog from './RemoveMediaFromAlbumDialog.vue';
import CarouselDialog from './CarouselDialog.vue';


@Component({
  components: {
    'blm-gallery-dialog-add-media-to-album': AddMediaToAlbumDialog,
    'blm-gallery-dialog-carousel': CarouselDialog,
    'blm-gallery-dialog-remove-media-from-album': RemoveMediaFromAlbumDialog,
    'blm-gallery-dialog-rename-album': RenameAlbumDialog,
  },
})
export default class Index extends Vue {
  // props
  // data
  media: any[] = [];
  album: any = null;
  is_loading = false;
  display_rename_album_dialog = false;
  display_add_media_to_album_dialog = false;
  display_remove_dialog = false;
  display_carousel_dialog = false;
  error = '';


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
      const res = await api.get(`${api.GALLERY}/v1/albums/${this.$route.params.album_id}`);
      this.media = res.media;
      this.album = res.album;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async delete_album() {
    if (this.album === null) {
      return;
    }
    this.error = '';
    this.is_loading = true;
    try {
      await api.delete(`${api.GALLERY}/v1/albums/${this.$route.params.album_id}`);
      router.push({ path: '/gallery/albums' });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  album_renamed(album: any) {
    this.album = album;
  }
  added_to_album() {
    this.fetch_data();
  }
  media_removed() {
    this.fetch_data();
  }

  open_rename_dialog() {
    this.display_rename_album_dialog = true;
  }
  open_add_dialog() {
    this.display_add_media_to_album_dialog = true;
  }
  open_remove_dialog() {
    this.display_remove_dialog = true;
  }
  open_carousel_dialog() {
    this.display_carousel_dialog = true;
  }
}
</script>


<style scoped lang="scss">
.add-btn {
  height: 36px !important;
}
</style>
