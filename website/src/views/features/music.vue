<template>
  <blm-feature-page
    name="Music"
    description="A world of emotion. <br/> Available wherever you are."
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
      description: `You can use Bloom Music to listen to your library on your computer,
      smartphone, or other supported devices.`,
      img: '/static/imgs/features/music_devices.svg',
      name: 'Play from any device',
    },
    {
      description: `Whether you've got plenty of rare tunes ripped from vinyls, a bunch of albums
        from indie artists on Bandcamp, or a carefully curated collection of lossless tracks,
        Bloom Music allows you to steam them from wherever you are.`,
      img: '/static/imgs/features/music_walking.svg',
      name: 'Stream your personal music library wherever you are',
    },
    {
      description: `Thanks to the unified data model of Bloom, all your music files in your Bloom
        Drive are directly available in Bloom Music. No need to reorganize everything everytime.`,
      img: '/static/imgs/features/music_organized.svg',
      name: 'Directly from your drive',
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>
