<template>
  <v-container class="mt-3 text-center">
    <v-row class="justify-center align-center">
      <v-col cols="12" sm="6" class="text-center">
        <h1 class="display-3 mb-5">{{ name }}</h1>

        <p v-html="description"></p>

        <blm-download-btn />

        <blm-other-downloads-link />
      </v-col>

      <v-col cols="12" sm="6" class="text-center mb-5">
        <v-img alt="hero" :src="hero" contain height="300">
          <template v-slot:placeholder>
            <v-row
              class="fill-height ma-0"
              align="center"
              justify="center"
            >
              <v-progress-circular indeterminate color="grey lighten-2" />
            </v-row>
          </template>
        </v-img>
      </v-col>
    </v-row>

    <v-row class="blm-features"></v-row>

    <v-row class="justify-center mb-5 mt-5 align-center blm-feature"
      v-for="(feature, index) in features" :key="index">
      <v-col cols="12" v-if="feature.name === 'dl_btn'">
        <div style="height: 100px;">
          <blm-download-btn outlined />
        </div>
      </v-col>

      <v-col cols="12" sm="6" class="pt-2 pb-2" v-if="feature.name !== 'dl_btn'"
        order="first" :order-sm="index % 2 === 0 ? 'first' : 'last'">
        <!-- <v-img :src="feature.img" contain height="200px"/> -->
        <img :src="feature.img" height="200" />
      </v-col>

      <v-col cols="12" sm="6" v-if="feature.name !== 'dl_btn'">
        <h3 class="display-1 font-weight-regular">
          {{ feature.name }}
        </h3>
        <p class="mt-5" v-html="feature.description"></p>
      </v-col>
    </v-row>

    <v-row class="justify-center mb-5 mt-5 align-center blm-dl">
      <v-col cols="12">
        <blm-download-btn outlined />
        <blm-other-downloads-link />
      </v-col>
    </v-row>

  </v-container>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import DownloadBtn from '@/components/coming_soon_btn.vue';
import OtherDownloadsLink from '@/components/other_downloads_link.vue';


@Component({
  components: {
    'blm-download-btn': DownloadBtn,
    'blm-other-downloads-link': OtherDownloadsLink,
  },
})
export default class FeaturePage extends Vue {
  // props
  @Prop({ type: String, required: true }) name!: string;
  @Prop({ type: String, required: true }) description!: string;
  @Prop({ type: Array, required: true }) features!: any[];
  @Prop({ type: String, required: true }) hero!: any[];


  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style lang="scss" scoped>
.blm-features {
  margin-top: 80px;
}

.blm-feature {
  margin-top: 42px;
}

.blm-dl {
  height: 150px;
}
</style>
