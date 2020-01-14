<template>
  <v-container fluid>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar flat dense>
      <v-spacer />
      <v-btn color="primary" @click="createAlbum">
        <v-icon left>mdi-plus</v-icon>New Album
      </v-btn>
    </v-toolbar>

    <v-container fluid>
      <v-row dense>
        <v-col cols="2" sm="4" lg="3" v-for="album in albums" :key="album.id">
          <router-link :to="`/gallery/albums/${album.id}`" slot="activator">
            <v-img src="/assets/imgs/gallery_album.svg" height="150px" contain class="app-icon"/>
            <p class="headline text-center app-name mt-3">{{ album.name }}</p>
          </router-link>
        </v-col>
      </v-row>

    </v-container>


    <blm-gallery-dialog-new-album
      :visible="isNewAlbumDialogVisible"
      @closed="closeNewAlbumDialog"
    />
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import NewAlbumDialog from '../components/new_album_dialog.vue';


@Component({
  components: {
    'blm-gallery-dialog-new-album': NewAlbumDialog,
  },
})
export default class Albums extends Vue {
  // props
  // data
  error = '';
  albums = [
    { name: 'Hello world', id: '1' },
  ];
  isNewAlbumDialogVisible = false;


  // computed
  // lifecycle
  // watch
  // methods
  createAlbum() {
    this.isNewAlbumDialogVisible = true;
  }

  closeNewAlbumDialog() {
    this.isNewAlbumDialogVisible = false;
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
