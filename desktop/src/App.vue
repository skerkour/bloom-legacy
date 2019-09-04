<template>
  <v-app :dark="darkMode">
    <component :is="layout">
      <router-view/>
    </component>
  </v-app>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import DefaultLayout from '@/bloom/kernel/layouts/Default.vue';
import AuthLayout from '@/bloom/kernel/layouts/Auth.vue';

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
  get darkMode() {
    return this.$store.state.dark_mode;
  }

  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss">
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
#nav {
  padding: 30px;
  a {
    font-weight: bold;
    color: #2c3e50;
    &.router-link-exact-active {
      color: #42b983;
    }
  }
}

a {
  text-decoration: none;
}

.blm-underline-hover:hover {
  text-decoration: underline;
}
</style>
