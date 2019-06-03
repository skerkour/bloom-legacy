<template>
  <v-list>

    <v-list-tile exact to="/myaccount">
      <v-list-tile-action>
        <v-icon>mdi-account</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Profile</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <v-list-tile exact to="/myaccount/billing">
      <v-list-tile-action>
        <v-icon>mdi-credit-card</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Billing</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <v-list-tile exact to="/myaccount/security">
      <v-list-tile-action>
        <v-icon>mdi-shield-check</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Security</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <v-list-tile exact to="/myaccount/devices">
      <v-list-tile-action>
        <v-icon>mdi-cellphone-link</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Devices</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <!-- <v-list-tile exact to="/myaccount/audit">
      <v-list-tile-action>
        <v-icon>mdi-format-list-bulleted</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Audit log</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile> -->

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
