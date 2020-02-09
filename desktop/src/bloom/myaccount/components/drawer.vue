<template>
  <v-list nav dense class="text-left">

    <v-list-item exact link to="/myaccount">
      <v-list-item-icon>
        <v-icon>mdi-account</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Profile</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/myaccount/security">
      <v-list-item-icon>
        <v-icon>mdi-shield-check</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Security</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/myaccount/billing">
      <v-list-item-icon>
        <v-icon>mdi-credit-card</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Billing</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/myaccount/devices">
      <v-list-item-icon>
        <v-icon>mdi-cellphone-link</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Devices</v-list-item-title>
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
