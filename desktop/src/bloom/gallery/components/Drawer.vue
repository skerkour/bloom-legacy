<template>
  <v-list nav dense class="text-left">

    <v-list-item exact link to="/gallery">
      <v-list-item-icon>
        <v-icon>mdi-image</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Gallery</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/gallery/albums">
      <v-list-item-icon>
        <v-icon>mdi-book-multiple</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Albums</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

  </v-list>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Drawer extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
