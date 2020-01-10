<template>
  <blm-feature-page
    name="Gallery"
    :description="$t('features.gallery.description')"
    hero="/static/imgs/features/gallery_hero.svg"
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
  // computed
  get features(): any[] {
    return [
      {
        description: this.$t('features.gallery.feature1.description'),
        img: '/static/imgs/features/gallery_devices.svg',
        name: this.$t('features.gallery.feature1.name'),
      },
      {
        description: this.$t('features.gallery.feature2.description'),
        img: '/static/imgs/features/gallery_anywhere.svg',
        name: this.$t('features.gallery.feature2.name'),
      },
      {
        description: this.$t('features.gallery.feature3.description'),
        img: '/static/imgs/features/gallery_organized.svg',
        name: this.$t('features.gallery.feature3.name'),
      },
    ];
  }
  // lifecycle
  // watch
  // methods
}
</script>
