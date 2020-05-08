<template>
  <v-container fill-height fluid class="pa-0">
    <v-row justify="center" align="center">
      <v-col cols="4" lg="3" class="pa-0 blm-left-col">
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

      <v-col cols="8" lg="9" class="pa-0">
        <blm-game v-if="selectedGameIndex !== undefined" :game="games[selectedGameIndex]" />
      </v-col>
    </v-row>
  </v-container>
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
  gameWindow = null;
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
    {
      icon: '/assets/imgs/icons/tron.jpg',
      name: 'Tron',
      description: `Engage in the most popular and deadly sport of of cyberspace, the light cycle
        arena. Leave lethal laser trails behind you and entrap your enemies while avoiding a similar
        fate in a 3D rendition of the classic science fiction movie scene.
      `,
      launch: '/arcade/games/tron/index.html',
    },
  ];

  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
.blm-left-col {
  border-right: 1px solid #dedede;
}
</style>
