<template>
  <blm-feature-page
    name="Gallery"
    description="All your media organized and easy to find"
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
  features = [
    {
      description: `You can access your galleries on your computer,
      smartphone, or other supported devices.`,
      img: '/static/imgs/features/gallery_devices.svg',
      name: 'Access from any device',
    },
    {
      description: 'Your photos will be safe, secure, and always with you.',
      img: '/static/imgs/features/gallery_anywhere.svg',
      name: 'Access your personal gallery wherever you are',
    },
    {
      description: `Thanks to the unified data model of Bloom, all your photos and videos files
        in your Bloom Drive are directly available in Bloom Gallery. No need to reorganize
        everything everytime.`,
      img: '/static/imgs/features/gallery_organized.svg',
      name: 'Directly from your drive',
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>
