<template>
  <!-- <v-container fluid>
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
  </v-container> -->
  <v-layout fill-height>
    <v-col cols="4" lg="3" class="pa-0">
      <div style="height: 100vh" class="overflow-y-auto">
        <v-list-item-group v-model="selectedGameIndex" color="indigo" mandatory>
          <v-list three-line class="pa-0">
            <template v-for="(game, index) in games" class="blm-pointer">
              <v-list-item :key="`note-${index}`">

                <v-list-item-avatar>
                  <v-img :src="game.icon"></v-img>
                </v-list-item-avatar>

                <v-list-item-content class="text-left">
                  <v-list-item-title>{{ game.name }}</v-list-item-title>
                  <v-list-item-subtitle>{{ game.description }}</v-list-item-subtitle>
                </v-list-item-content>

              </v-list-item>
              <v-divider v-if="index !== games.length - 1" :key="index"/>
            </template>
          </v-list>
        </v-list-item-group>
      </div>
    </v-col>

    <v-col cols="8" lg="9" class="pa-0 blm-main-col">
      <blm-game :game="games[0]" />
    </v-col>
  </v-layout>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmGame from '../components/game.vue';

export type Game = {
  icon: string,
  name: string,
  description: string,
  launch: string,
}


@Component({
  components: {
    BlmGame,
  },
})
export default class BlmGames extends Vue {
  // props
  // data
  window2048 = null;
  selectedGameIndex = 0;
  games: Game[] = [
    {
      icon: '/assets/imgs/icons/2048.png',
      name: '2048',
      description: `2048 is a single-player sliding block puzzle game designed by the Italian web
        developer Gabriele Cirulli. The game's objective is to slide numbered tiles on a grid to
        combine them to create a tile with the number 2048.
      `,
      launch: '/arcade/games/2048/index.html',
    },
  ];

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
.blm-main-col {
  border-left: 1px solid #dedede;
}
</style>
