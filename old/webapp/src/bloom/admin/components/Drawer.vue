<template>
  <v-list>

    <v-list-tile exact to="/admin">
      <v-list-tile-action>
        <v-icon>mdi-view-dashboard</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Dashboard</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <v-list-tile exact to="/admin/accounts">
      <v-list-tile-action>
        <v-icon>mdi-account</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Accounts</v-list-tile-title>
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
