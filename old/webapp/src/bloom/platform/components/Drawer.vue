<template>
  <div>
    <PhaserDrawer v-if="subservice === 'phaser'" />
    <v-list v-else>

    <v-list-tile exact to="/platform/phaser">
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
import PhaserDrawer from '@/bloom/phaser/components/Drawer.vue';


@Component({
  components: {
    PhaserDrawer,
  },
})
export default class PlatformDrawer extends Vue {
  // props
  // data
  // computed
  get subservice() {
    if (this.$route.path.indexOf('/platform/phaser') === 0) {
      return 'phaser';
    }

    return 'dashboard';
  }
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
</style>
