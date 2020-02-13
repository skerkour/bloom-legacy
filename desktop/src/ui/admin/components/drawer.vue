<template>
  <v-list nav dense class="text-left">

    <v-list-item exact link to="/admin">
      <v-list-item-icon>
        <v-icon>mdi-view-dashboard</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Dashboard</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/admin/users">
      <v-list-item-icon>
        <v-icon>mdi-account</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Users</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/admin/groups">
      <v-list-item-icon>
        <v-icon>mdi-account-group</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Groups</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/admin/plans">
      <v-list-item-icon>
        <v-icon>mdi-currency-eur</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Plans</v-list-item-title>
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
