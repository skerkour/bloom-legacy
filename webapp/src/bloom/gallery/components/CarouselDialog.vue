<template>
  <v-dialog v-model="show" fullscreen hide-overlay @keydown.esc="show = false">
  <v-card>
    <v-toolbar dark color="black">
      <v-btn icon dark @click="show = false">
        <v-icon>mdi-close</v-icon>
      </v-btn>
    </v-toolbar>

    <v-card-text class="text-xs-center">
      <v-carousel :cycle="false" height="100%" v-model="_index" hide-delimiters>
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
  // watch
  // methods
}
</script>


<style scoped lang="scss">
.v-card {
  background-color: black;
}

.blm-carousel-image {
  height: 100%;
}
</style>
