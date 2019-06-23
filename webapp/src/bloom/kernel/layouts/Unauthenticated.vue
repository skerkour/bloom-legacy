<template>
  <div id="layout-wrapper">
    <v-navigation-drawer
      fixed
      clipped
      app
      width="250"
      v-if="$route.meta.service === 'help'">
      <blm-help-drawer />
    </v-navigation-drawer>
    <blm-toolbar />
    <v-content>
      <router-view></router-view>
    </v-content>
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import HelpDrawer from '@/bloom/help/components/Drawer.vue';


@Component({
  components: {
    'blm-help-drawer': HelpDrawer,
  },
})
export default class Unauthenticated extends Vue {
  // props
  // data
  display_btns = false;


  // computed
  // lifecycle
  created() {
    if (this.$route.path.indexOf('/recovery/request') === 0) {
      this.display_btns = true;
    }
  }
  // watch
  // methods
}
</script>


<style scoped lang="scss">

.v-toolbar a {
  color: white;
  text-decoration: none;
}

#menu-account {
  margin-left: calc(50% - 44px);
}

#layout-wrapper {
  height: 100%;
}

.v-content {
  height: 100%;
}
</style>
