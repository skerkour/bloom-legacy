<template>
<div>
  <v-layout>
    <v-flex xs12 sm9 offset-sm1>
      <div class="elevation-0">
        <v-container grid-list-sm fluid>
          <v-layout row wrap>
             <v-flex v-if="media.length === 0" xs12>
              <v-container class="text-xs-center">
                <p>No media</p>
              </v-container>
            </v-flex>
            <v-flex v-for="file in media" :key="file.id" xs4 sm3 md2 d-flex
             @click="open_dialog_caroussel" class="pointer">
              <v-card flat tile class="d-flex">
                <v-img :src="file.url" class="grey lighten-2" contain>
                  <template v-slot:placeholder>
                    <v-layout fill-height align-center justify-center ma-0>
                      <v-progress-circular indeterminate color="grey lighten-5"></v-progress-circular>
                    </v-layout>
                  </template>
                </v-img>
              </v-card>
            </v-flex>
          </v-layout>
        </v-container>
      </div>
    </v-flex>
  </v-layout>
  <blm-gallery-dialog-carousel
    :media="media"
    :visible="display_carousel_dialog"
    @close="display_carousel_dialog = false" />
</div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import DialogCarousel from './CarouselDialog.vue';

@Component({
  components: {
    'blm-gallery-dialog-carousel': DialogCarousel,
  },
})
export default class Index extends Vue {
  // props
  // data
  media: any[] = [];
  is_loading = false;
  error = '';
  display_carousel_dialog = false;


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
      this.media = await api.get(`${api.GALLERY}/v1/media`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  open_dialog_caroussel() {
    this.display_carousel_dialog = true;
  }
}
</script>


<style scoped lang="scss">
</style>
