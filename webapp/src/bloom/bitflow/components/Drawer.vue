<template>
  <v-list>

    <v-list-tile exact to="/bitflow">
      <v-list-tile-action>
        <v-icon>mdi-cloud-download</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Downloads</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <v-list-tile exact to="/bitflow/history">
      <v-list-tile-action>
        <v-icon>mdi-history</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>History</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

  </v-list>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

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
