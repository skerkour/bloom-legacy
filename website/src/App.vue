<template>
  <v-app>
    <blm-app-bar />
    <v-content>
      <router-view/>
    </v-content>
    <blm-footer />
  </v-app>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import AppBar from '@/components/app_bar.vue';
import Footer from '@/components/footer.vue';


@Component({
  components: {
    'blm-app-bar': AppBar,
    'blm-footer': Footer,
  },
})
export default class App extends Vue {}
</script>
