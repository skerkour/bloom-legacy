<template>
  <div>
    <v-list>

      <v-list-tile exact to="/notes">
        <v-list-tile-action>
          <v-icon>mdi-note</v-icon>
        </v-list-tile-action>
        <v-list-tile-content>
          <v-list-tile-title>Notes</v-list-tile-title>
        </v-list-tile-content>
      </v-list-tile>

      <v-divider></v-divider>


      <v-list-tile exact to="/notes/archive">
        <v-list-tile-action>
          <v-icon>mdi-package-down</v-icon>
        </v-list-tile-action>
        <v-list-tile-content>
          <v-list-tile-title>Archive</v-list-tile-title>
        </v-list-tile-content>
      </v-list-tile>

      <v-list-tile exact to="/notes/trash">
        <v-list-tile-action>
          <v-icon>mdi-delete</v-icon>
        </v-list-tile-action>
        <v-list-tile-content>
          <v-list-tile-title>Trash</v-list-tile-title>
        </v-list-tile-content>
      </v-list-tile>

    </v-list>
  </div>
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


<style scoped lang="scss">
</style>
