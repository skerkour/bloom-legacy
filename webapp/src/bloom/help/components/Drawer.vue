<template>
  <div>
    <v-list>

      <v-list-tile exact to="/help">
        <v-list-tile-action>
          <v-icon color="grey darken-1">mdi-home</v-icon>
        </v-list-tile-action>
        <v-list-tile-content>
          <v-list-tile-title>Home</v-list-tile-title>
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
