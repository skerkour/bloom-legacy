<template>
  <v-container class="mt-3 text-center">
    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <h1 class="display-3 mb-5">
          {{ name }}
        </h1>

        <p>
          {{ description }}
        </p>

        <blm-download-btn />

        <p class="subtitle-1 mt-5"><router-link to="/download" class="text--darken-1 grey--text">
          Other downloads
        </router-link></p>
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
        <v-img :src="feature.img" contain height="200px"/>
      </v-col>

      <v-col cols="12" sm="6" v-if="feature.name !== 'dl_btn'">
        <h3 class="display-1 font-weight-regular">
          {{ feature.name }}
        </h3>
        <p class="mt-5" v-html="feature.description"></p>
      </v-col>
    </v-row>

  </v-container>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import DownloadBtn from '@/components/download_btn.vue';


@Component({
  components: {
    'blm-download-btn': DownloadBtn,
  },
})
export default class FeaturePage extends Vue {
  // props
  @Prop({ type: String, required: true }) name!: string;
  @Prop({ type: String, required: true }) description!: string;
  @Prop({ type: Array, required: true }) features!: any[];

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
</style>
