<template>
  <v-container fluid>
    <v-card
      class="mx-auto"
      max-width="200"
      outlined
      href="/arcade/games/2048/index.html"
    >

    <v-img
      src="/assets/imgs/icons/2048.png"
      height="200px"
      width="200px"
    ></v-img>

    <v-card-title>2048</v-card-title>

    </v-card>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


@Component
export default class Index extends Vue {
  // props
  // data
  window2048 = null;

  // computed
  // lifecycle
  // watch
  // methods
  launch2048() {
    // this.window2048 = new BrowserWindow();
    // this.window2048.on('closed', () => {
    //   this.window2048 = null;
    // });

    // this.window2048.loadURL(`file://${__dirname}/public/arcade/games/2048/index.html`);
  }
}
</script>


<style lang="scss" scoped>
</style>
