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


<style lang="scss">
// Global vars
$grey: #2c3e50;
$white: #fff;
$red: #F44336;
$blue: #2196F3;

$success: #67C23A;
$info: #409EFF;
$warning: #E6A23C;
$danger: #F56C6C;

@font-face {
  font-family: "rounded_elegance";
  src: url("/fonts/rounded_elegance.ttf") format("truetype");
}
</style>
