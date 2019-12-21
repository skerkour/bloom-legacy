<template>
  <v-container fluid>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar flat dense>
      <v-toolbar-title>{{ album.name }} </v-toolbar-title>
      <v-spacer />

      <v-btn color="primary" @click="addSongs">
        <v-icon left>mdi-plus</v-icon>Add media
      </v-btn>

      <v-btn color="red" text class="mx-2" @click="removeSongs">
        <v-icon left>mdi-minus</v-icon>Remove media
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
          <v-list-item @click="renameAlbum">
            <v-list-item-icon>
              <v-icon>mdi-alphabetical</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Rename Album</v-list-item-title>
          </v-list-item>
          <v-list-item @click="deleteAlbum">
            <v-list-item-icon>
              <v-icon>mdi-delete</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Delete Album</v-list-item-title>
          </v-list-item>
      </v-list>

      </v-menu>
    </v-toolbar>

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Albums extends Vue {
  // props
  // data
  error = '';
  media = [];
  album = {
    name: 'Hello World',
  };
  // computed
  // lifecycle
  // watch
  // methods
  renameAlbum() {
  }

  deleteAlbum() {
  }

  addSongs() {
  }

  removeSongs() {
  }
}
</script>


<style lang="scss" scoped>
</style>
