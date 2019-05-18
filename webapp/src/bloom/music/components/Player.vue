<template>
<v-card class="elevation-3">
  <v-slider
    id="player-slider" v-model="slider" height="5px" hide-details @change="update_seek"
    step="0"/>
  <v-container flat text-xs-center>
    <v-layout row wrap justify-center align-center>
      <v-flex xs12>
        <div v-if="playing">{{ playing.name }}</div>
      </v-flex>

      <v-flex xs12>
        <v-btn flat icon @click="toggle_repeat()"
          :color="repeat_state !== repeatState.ALL ? 'amber' : undefined">
          <v-icon dark v-if="repeat_state === repeatState.ALL">mdi-repeat</v-icon>
          <v-icon dark v-else-if="repeat_state === repeatState.ONCE">mdi-repeat-once</v-icon>
          <v-icon dark v-else>mdi-repeat-off</v-icon>
        </v-btn>

        <v-btn flat icon @click="previous()">
          <v-icon dark>mdi-skip-previous</v-icon>
        </v-btn>

        <v-btn flat icon @click="toggle_play_pause">
          <v-icon v-if="this.is_playing">mdi-pause</v-icon>
          <v-icon v-else>mdi-play</v-icon>
        </v-btn>

        <v-btn flat icon @click="next()">
          <v-icon dark>mdi-skip-next</v-icon>
        </v-btn>

        <v-btn flat icon @click="toggle_shuffle()" :color="is_shuffle ? 'amber' : undefined">
          <v-icon dark>mdi-shuffle</v-icon>
        </v-btn>
      </v-flex>
    </v-layout>
  </v-container>
</v-card>
</template>

<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import {Howl, Howler} from 'howler';
import api from '@/bloom/kernel/api';


export enum RepeatState {
  ALL,
  ONCE,
  NONE,
}


@Component
export default class DialogAddSongsToPlaylist extends Vue {
  // props
  @Prop({ type: Array, default: [] }) musics!: any[];
  @Prop({ type: Number, default: 0 }) index!: number;

  // data
  error = '';
  playing: any = null;
  is_playing = false;
  _index = 0;
  is_shuffle = false;
  repeat_state = RepeatState.ALL;
  slider = 0;
  seek_interval: any = null;


  // computed
  get repeatState() {
    return RepeatState;
  }


  // lifecycle
  created() {
    this._index = this.index;
    this.playing = this.musics[this._index];
  }

  destroyed() {
    if (this.playing !== null) {
      this.playing.sound.stop();
    }
  }

  // watch
  @Watch('index')
  on_index_changer(to: number) {
    this.stop();
    this._index = to;
    this.play();
  }


  // methods
  play() {
    if (this._index < 0 || this._index >= this.musics.length) {
      this._index = 0;
    }
    this.playing = this.musics[this._index];
    this.playing.sound = new Howl({
      html5: true,
      preload: false,
      src: [this.playing.url],
    });
    this.playing.sound.play();
    this.playing.sound.on('end', () => {
      this.next();
      this.is_playing = false;
    });
    this.seek_interval = setInterval(() => {
      this.slider =  (this.playing.sound.seek() / this.playing.sound.duration()) * 100;
    }, 250);
    this.is_playing = true;
  }

  resume() {
    if (this.playing) {
      this.playing.sound.play();
      this.playing.sound.seek(this.playing.position);
      this.is_playing = true;
    } else {
      this.play();
    }
  }

  next() {
    // this.stop();
    let index = this._index;

    if (this.is_shuffle) {
      index = this.random();
    } else {
      switch (this.repeat_state) {
        case RepeatState.ONCE:
          break;
        case RepeatState.ALL:
          if (index + 1 >= this.musics.length) {
            index = 0;
          } else {
            index += 1;
          }
          break;
        case RepeatState.NONE:
          if (index + 1 >= this.musics.length) {
            return;
          } else {
            index += 1;
          }
      }
    }

    this.$emit('update', index);
    // this.play();
  }

  previous() {
    // this.stop();
    let index = this._index;

    if (this.is_shuffle) {
      index = this.random();
    } else {
      if (index - 1 < 0) {
        index = this.musics.length - 1;
      } else {
        index -= 1;
      }
    }

    this.$emit('update', index);
    // this.play();
  }

  toggle_repeat() {
    switch (this.repeat_state) {
      case RepeatState.ALL:
        this.repeat_state = RepeatState.ONCE;
        break;
      case RepeatState.ONCE:
        this.repeat_state = RepeatState.NONE;
        break;
      case RepeatState.NONE:
        this.repeat_state = RepeatState.ALL;
    }
  }

  toggle_shuffle() {
    this.is_shuffle = !this.is_shuffle;
  }

  toggle_play_pause() {
    if (this.is_playing) {
      this.pause();
    } else {
      this.resume();
    }
  }

  pause() {
    if (this.is_playing) {
      this.playing.sound.pause();
      this.playing.position = this.playing.sound.seek();
      this.is_playing = false;
    }
  }

  stop() {
    if (this.is_playing) {
      this.playing.sound.stop();
      this.is_playing = false;
      clearInterval(this.seek_interval);
      this.seek_interval = null;
    }
  }

  random(): number {
    const min = 0;
    const max = this.musics.length - 1;
    return Math.floor(Math.random() * (max - min + 1) + min);
  }

  update_seek(new_value: number) {
    if (this.is_playing) {
      this.playing.sound.seek((this.playing.sound.duration() / 100) * new_value);
    } else {
      this.slider = 0;
    }
  }
}
</script>


<style scoped lang="scss">
#player-slider {
  margin-top: -2px;
}
</style>
