<template>
  <v-list>
    <BackToPlatformDashboard />

    <v-list-tile exact to="/platform/phaser">
      <v-list-tile-action>
        <v-icon>mdi-format-list-bulleted</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Scans</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <!-- <v-list-tile exact to="/platform/phaser/profiles">
      <v-list-tile-action>
        <v-icon>mdi-account</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Profiles</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile> -->
  </v-list>
</template>

<script>
import BackToPlatformDashboard from '@/bloom/platform/components/BackToPlatformDashboard.vue';

export default {
  components: {
    BackToPlatformDashboard,
  },
};
</script>
