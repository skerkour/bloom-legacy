<template>
  <blm-feature-page
    name="Music"
    :description="$t('features.music.description')"
    hero="/static/imgs/features/music_hero.svg"
    :features="features"
  />
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import FeaturePage from '@/components/feature_page.vue';


@Component({
  components: {
    'blm-feature-page': FeaturePage,
  },
})
export default class Drive extends Vue {
  // props
  // data
  get features(): any[] {
    return [
      {
        description: this.$t('features.music.feature1.description'),
        img: '/static/imgs/features/music_devices.svg',
        name: this.$t('features.music.feature1.name'),
      },
      {
        description: this.$t('features.music.feature2.description'),
        img: '/static/imgs/features/music_walking.svg',
        name: this.$t('features.music.feature2.name'),
      },
      {
        description: this.$t('features.music.feature3.description'),
        img: '/static/imgs/features/music_organized.svg',
        name: this.$t('features.music.feature3.name'),
      },
    ];
  }
  // computed
  // lifecycle
  // watch
  // methods
}
</script>
