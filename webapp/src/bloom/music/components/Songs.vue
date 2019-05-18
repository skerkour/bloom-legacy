<template>
<div>
  <v-layout>
    <v-flex xs12>
      <v-card class="elevation-0 tracks-table">

          <v-data-table
            :headers="headers"
            :items="musics"
            :loading="is_loading"
            hide-actions
          >
            <template slot="no-data">
              <p class="text-xs-center">
                No Music.
              </p>
            </template>
            <v-progress-linear v-slot:progress color="blue" indeterminate></v-progress-linear>
            <template v-slot:items="props">
              <tr class="pointer"
                :active="props.index === index"
                @click="props.selected = !props.selected"
                v-on:dblclick="play_music(props.index)">
                  <td class="text-xs-left">
                  <v-layout align-center row fill-height>
                    <span>{{ props.item.name }}</span>
                  </v-layout>
                </td>
              </tr>
            </template>
          </v-data-table>

      </v-card>
    </v-flex>
  </v-layout>

  <blm-music-player id="music-player" :musics="musics" :index="index" @update="play_music"/>
</div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import Player from './Player.vue';


@Component({
  components: {
    'blm-music-player': Player,
  },
})
export default class Index extends Vue {
  // props
  // data
  musics: any[] = [];
  is_loading = false;
  error = '';
  index = -1;
  headers = [
    {
      align: 'left',
      sortable: false,
      text: 'Track',
      value: 'name',
    },
  ];


  // computed
  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    try {
      this.musics = await api.get(`${api.MUSIC}/v1/musics`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  play_music(index: number) {
    this.index = index;
  }
}
</script>


<style scoped lang="scss">
#music-player {
  position: fixed;
  bottom: 0px;
  width: calc(100% - 250px);

  @media screen and (max-width: 1264px) {
    width: 100%;
  }
}

.tracks-table {
  margin-bottom: 125px;
}
</style>
