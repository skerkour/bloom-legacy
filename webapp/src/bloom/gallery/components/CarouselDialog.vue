<template>
  <v-dialog v-model="show" fullscreen hide-overlay @keydown.esc="show = false"
    v-on:keyup.right="next" v-on:keyup.left="previous">
  <v-card>
    <v-toolbar dark color="black">
      <v-btn icon dark @click="show = false">
        <v-icon>mdi-close</v-icon>
      </v-btn>
    </v-toolbar>

    <v-card-text class="text-xs-center">
      <v-carousel :cycle="false" height="calc(100vh - 120px)" v-model="_index" hide-delimiters>
        <v-carousel-item v-for="file in media" :key="file.id" lazy>
          <v-img :src="file.url" contain height="100%" />
        </v-carousel-item>
      </v-carousel>
    </v-card-text>
  </v-card>
</v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


const LEFT = 37;
const RIGHT = 39;

@Component
export default class CreateAlbumDialog extends Vue {
  // props
  @Prop({ type: Array, default: [] }) media!: any[];
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: Number, default: false }) index!: number;


  // data
  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('close');
    }
  }

  get _index(): number {
    return this.index;
  }

  set _index(value: number) {
    this.$emit('update', value);
  }


  // lifecycle
  mounted() {
    window.addEventListener('keyup', this.on_key_up);
  }

  destroyed() {
    window.removeEventListener('keyup', this.on_key_up);
  }
  // watch
  // methods
  next() {
    if (this.index + 1 >= this.media.length) {
      this._index = 0;
    } else {
      this._index += 1;
    }
  }

  previous() {
    if (this.index - 1 < 0) {
      this._index = this.media.length - 1;
    } else {
      this._index -= 1;
    }
  }

  on_key_up(e: any) {
    if (e.keyCode === LEFT) {
      this.previous();
    } else if (e.keyCode === RIGHT) {
      this.next();
    }
  }
}
</script>


<style scoped lang="scss">
.v-card {
  background-color: black;
}

.v-image__image--contain {
  background-position: top center !important;
}
</style>
