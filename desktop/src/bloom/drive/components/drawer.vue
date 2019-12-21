<template>
  <v-list nav dense class="text-left">

    <v-list-item exact to="/drive">
      <v-list-item-icon>
        <v-icon>mdi-desktop-tower</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>My Drive</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact to="/drive/trash">
      <v-list-item-icon>
        <v-icon>mdi-delete</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Trash</v-list-item-title>
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
