<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4">
        <v-card elevation="0" class="text-center justify-center">
          <v-card-text class="pa-0 justify-center">
            <img :src="game.icon" width="256"/>
            <h2 class="headline mt-2">{{ game.name }}</h2>
            <p class="mt-3">{{ game.description }}</p>
          </v-card-text>
          <v-card-actions class="justify-center">
            <v-btn color="success" depressed @click="launchGame()">
              <v-icon left>mdi-play</v-icon> Play
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import { Game } from '../views/games.vue';

@Component
export default class BlmGame extends Vue {
  // props
  @Prop({ type: Object, required: true }) game!: Game;

  // data
  // computed
  // lifecycle
  // watch
  // methods
  launchGame() {
    window.location = this.game.launch as any;
  }
}
</script>


<style lang="scss" scoped>
</style>
