<template>
  <v-dialog
  v-model="show"
  fullscreen hide-overlay
  @keydown.esc="show = false">
  <v-card>
    <v-toolbar dark color="black">
      <v-btn icon dark @click="show = false">
        <v-icon>mdi-close</v-icon>
      </v-btn>
    </v-toolbar>

    <v-card-text class="text-xs-center">
     <v-carousel>
      <v-carousel-item
        v-for="file in media"
        :key="file.id"
        :src="file.url"
        cycle="false"
      ></v-carousel-item>
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


  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
.v-card {
  background-color: black;
}
</style>
