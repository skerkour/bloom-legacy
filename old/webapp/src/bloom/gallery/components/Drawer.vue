<template>
  <v-list>

    <v-list-tile exact to="/gallery">
      <v-list-tile-action>
        <v-icon>mdi-image</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Gallery</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <v-list-tile exact to="/gallery/albums">
      <v-list-tile-action>
        <v-icon>mdi-book-multiple-variant</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Albums</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

  </v-list>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
</style>
