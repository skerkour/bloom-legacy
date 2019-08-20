<template>
  <v-list>

    <v-list-tile exact to="/music">
      <v-list-tile-action>
        <v-icon>mdi-music</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Songs</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <v-list-tile exact to="/music/playlists">
      <v-list-tile-action>
        <v-icon>mdi-library-music</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Playlists</v-list-tile-title>
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
