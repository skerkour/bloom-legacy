<template>
  <v-list nav dense class="text-left">

    <v-list-item exact link to="/notes">
      <v-list-item-icon>
        <v-icon>mdi-note</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Notes</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/notes/archive">
      <v-list-item-icon>
        <v-icon>mdi-package-down</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Archive</v-list-item-title>
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
