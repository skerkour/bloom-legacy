<template>
  <v-navigation-drawer
    fixed
    floating
    clipped
    app
    width="200"
    permanent
    class="blm-navigation-drawer"
    v-if="showDrawer"
  >

    <!-- <div class="blm-drawer-space"></div> -->

    <blm-drawer-bitflow v-if="app === 'bitflow'" />
    <blm-drawer-files v-else-if="app === 'files'" />
    <blm-drawer-phaser v-else-if="app === 'phaser'" />
    <blm-drawer-admin v-else-if="app === 'admin'" />
    <blm-drawer-preferences v-else-if="app === 'preferences'" />
    <blm-drawer-myaccount v-else-if="app === 'myaccount'" />
    <blm-drawer-groups v-else-if="app === 'groups'" />

  </v-navigation-drawer>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import BitflowDrawer from '@/ui/bitflow/components/drawer.vue';
import FilesDrawer from '@/ui/files/components/drawer.vue';
import AdminDrawer from '@/ui/admin/components/drawer.vue';
import PreferencesDrawer from '@/ui/preferences/components/drawer.vue';
import MyAccountDrawer from '@/ui/myaccount/components/drawer.vue';
import GroupsDrawer from '@/ui/groups/components/drawer.vue';

const APPS_WITH_DRAWER = [
  'bitflow',
  'files',
  'admin',
  'preferences',
  'myaccount',
  'groups',
];

@Component({
  components: {
    'blm-drawer-bitflow': BitflowDrawer,
    'blm-drawer-files': FilesDrawer,
    'blm-drawer-admin': AdminDrawer,
    'blm-drawer-preferences': PreferencesDrawer,
    'blm-drawer-myaccount': MyAccountDrawer,
    'blm-drawer-groups': GroupsDrawer,
  },
})
export default class NavigationDrawer extends Vue {
  // props
  @Prop({ type: String, default: '' }) app!: string;


  // data
  // computed
  get showDrawer(): boolean {
    if (this.app === 'groups') {
      if (this.$route.params.groupId) {
        return true;
      }
      return false;
    }

    return APPS_WITH_DRAWER.includes(this.app);
  }


  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
.blm-navigation-drawer {
  margin-left: 72px;
}

.blm-drawer-space {
  height: 64px;

  @media screen and (max-width: 960px) {
    height: 56px;
  }
}
</style>
