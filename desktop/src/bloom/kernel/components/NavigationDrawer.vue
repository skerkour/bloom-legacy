<template>
  <v-navigation-drawer
    fixed
    floating
    clipped
    app
    width="250"
    permanent
    class="blm-navigation-drawer"
    v-if="showDrawer"
  >

    <div class="blm-drawer-space"></div>

    <blm-drawer-music v-if="app === 'music'" />
    <blm-drawer-bitflow v-else-if="app === 'bitflow'" />
    <blm-drawer-notes v-else-if="app === 'notes'" />
    <blm-drawer-drive v-else-if="app === 'drive'" />
    <blm-drawer-phaser v-else-if="app === 'phaser'" />
    <blm-drawer-gallery v-else-if="app === 'gallery'" />
    <blm-drawer-admin v-else-if="app === 'admin'" />

  </v-navigation-drawer>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import MusicDrawer from '@/bloom/music/components/drawer.vue';
import BitflowDrawer from '@/bloom/bitflow/components/drawer.vue';
import NotesDrawer from '@/bloom/notes/components/Drawer.vue';
import DriveDrawer from '@/bloom/drive/components/Drawer.vue';
import PhaserDrawer from '@/bloom/phaser/components/Drawer.vue';
import GalleryDrawer from '@/bloom/gallery/components/drawer.vue';
import AdminDrawer from '@/bloom/admin/components/drawer.vue';

const APPS_WITH_DRAWER = ['music', 'bitflow', 'notes', 'drive', 'phaser', 'gallery', 'admin'];

@Component({
  components: {
    'blm-drawer-music': MusicDrawer,
    'blm-drawer-bitflow': BitflowDrawer,
    'blm-drawer-notes': NotesDrawer,
    'blm-drawer-drive': DriveDrawer,
    'blm-drawer-phaser': PhaserDrawer,
    'blm-drawer-gallery': GalleryDrawer,
    'blm-drawer-admin': AdminDrawer,
  },
})
export default class NavigationDrawer extends Vue {
  // props
  @Prop({ type: String, default: '' }) app!: string;


  // data
  // computed
  get showDrawer() {
    return APPS_WITH_DRAWER.includes(this.app);
  }


  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
.blm-navigation-drawer {
  margin-left: 80px;
}

.blm-drawer-space {
  height: 64px;

  @media screen and (max-width: 960px) {
    height: 56px;
  }
}
</style>
