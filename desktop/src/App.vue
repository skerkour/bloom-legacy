<template>
  <v-app :dark="$store.state.darkMode">
    <component :is="layout">
      <router-view/>
    </component>
  </v-app>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import DefaultLayout from '@/ui/layouts/Default.vue';
import AuthLayout from '@/ui/layouts/Auth.vue';

@Component({
  components: {
    'blm-layout-auth': AuthLayout,
    'blm-layout-default': DefaultLayout,
  },
})
export default class App extends Vue {
  // props
  // data
  // computed
  get layout(): string {
    const layout = this.$route.meta.layout
      ? `blm-layout-${this.$route.meta.layout}` : 'blm-layout-default';
    return layout;
  }

  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss">
@font-face {
  font-family: "rounded_elegance";
  src: url("/assets/fonts/rounded_elegance.ttf") format("truetype");
}

#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  // color: #2c3e50;
}

a {
  text-decoration: none;
}

.v-btn {
  border-radius: 4px;
  text-transform: none !important; /* overwrite default 'uppercase' */
}

.blm-pointer {
  cursor: pointer;
}

.blm-underline-hover:hover {
  text-decoration: underline;
}

.theme--light.v-application {
  background-color: #fff !important;
}

.theme--dark.v-application {
  background-color: #424242 !important;
}
// .theme--light {
//   .v-toolbar, .v-table, .v-footer, .v-tabs__bar {
//     background-color: #fff;
//   }

//   pre {
//     background-color: #f0f0f0;
//   }

//   .v-application {
//     background-color: #fff !important;
//   }
// }
</style>
