<template>
  <v-container>
    <v-row dense>
        <v-col cols="2" sm="3" md="2" v-for="(file, index) in media" :key="file.id"
            @click="openCarousel(index)" class="pointer">
          <v-img :src="file.url" contain />
        </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Index extends Vue {
  // props
  // data
  media = [];
  // computed
  // lifecycle
  // watch
  // methods
  openCarousel() {
  }
}
</script>


<style lang="scss" scoped>
</style>
