<template>
  <div>
    <v-list>

    <v-list-tile exact to="/help">
      <v-list-tile-action>
        <v-icon color="grey darken-1">mdi-book-open-variant</v-icon>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Home</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <v-list-tile exact to="/help/drive">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/kernel/static/imgs/logos/drive_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Drive</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>


    <v-list-tile exact to="/help/platform">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/kernel/static/imgs/logos/platform_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Platform</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

    <v-list-tile exact to="/help/phaser">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/phaser/static/imgs/phaser_filled_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Phaser</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>


    <v-list-tile exact to="/help/myaccount">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/kernel/static/imgs/logos/myaccount_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>MyAccount</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>


    <v-list-tile exact to="/help/bitflow">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/kernel/static/imgs/logos/bitflow_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Bitflow</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>


    <v-list-tile exact to="/help/music">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/kernel/static/imgs/logos/music_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Music</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>


    <v-list-tile exact to="/help/gallery">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/kernel/static/imgs/logos/gallery_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Gallery</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>



    <v-list-tile exact to="/help/contacts">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/kernel/static/imgs/logos/contacts_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Contacts</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>


    <v-list-tile exact to="/help/arcade">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/kernel/static/imgs/logos/arcade_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Arcade</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>


    <v-list-tile exact to="/help/calendar">
      <v-list-tile-action>
        <v-avatar :size="32"><img src="/kernel/static/imgs/logos/calendar_64.png" /></v-avatar>
      </v-list-tile-action>
      <v-list-tile-content>
        <v-list-tile-title>Calendar</v-list-tile-title>
      </v-list-tile-content>
    </v-list-tile>

  </v-list>
  </div>
</template>



<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component
export default class HelpDrawer extends Vue {
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
