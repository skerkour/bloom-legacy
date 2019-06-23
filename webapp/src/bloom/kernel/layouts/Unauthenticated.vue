<template>
  <div id="layout-wrapper">
    <v-navigation-drawer
      v-model="drawer"
      fixed
      clipped
      app
      width="250"
      v-if="$route.meta.service === 'help'">

      <v-toolbar color="primary" class="hidden-md-and-up">
        <v-toolbar-side-icon dark @click.stop="drawer = !drawer" >
          <v-icon>mdi-menu</v-icon>
        </v-toolbar-side-icon>
      </v-toolbar>
      <blm-help-drawer />
    </v-navigation-drawer>

    <blm-toolbar  @click:side-icon="drawer = !drawer" />
    <v-content>
      <router-view></router-view>
    </v-content>
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import HelpDrawer from '@/bloom/help/components/Drawer.vue';
import { Route } from 'vue-router';


@Component({
  components: {
    'blm-help-drawer': HelpDrawer,
  },
})
export default class Unauthenticated extends Vue {
  // props
  // data
  display_btns = false;
  drawer = false;

  // computed
  // lifecycle
  created() {
    if (this.$route.path.indexOf('/recovery/request') === 0) {
      this.display_btns = true;
    }
    this.setup(false);
  }


  // watch
  @Watch('$route')
  on_route_change(_: Route, prev: Route) {
    let wasOpen = false;
    if (typeof prev.meta.service === 'string') {
      wasOpen = true;
    }

    this.setup(wasOpen);
  }


  // methods
    // TODO: close drawer on homepage resize
  setup(wasOpen: boolean) {
    if (typeof this.$route.meta.service !== 'string' && this.$route.meta.service !== '') {
      this.drawer = false; // close for homepage
    }
    if (this.$vuetify.breakpoint.mdAndDown || wasOpen) {
      return; // do not reopen on every route change within each service route
    }

    switch (this.$route.meta.service) {
      case 'help':
        this.drawer = true;
        break;
      default:
        this.drawer = false;
        break;
    }
  }
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
