<template>
  <blm-feature-page
    name="Bitflow"
    description="Download torrents to a secured cloud. Fast. Online."
    hero="/static/imgs/features/bitflow_hero.svg"
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
      description: `Your files are downloaded directly to your Drive and can can be accessed from
        any smartphone, tablet, or computer. So wherever you go, your downloaded files follow.`,
      img: '/static/imgs/features/bitflow_devices.svg',
      name: 'Download anywhere, anytime',
    },
    {
      description: `With Bitflow you don’t need to download any program, all you need is a
      browser. Bitflow currently supports Torrent and HTTP(S) protocols.`,
      img: '/static/imgs/features/bitflow_protocols.svg',
      name: 'No need to install a torrent client',
    },
    {
      description: `It takes a long time to download large files with a residential bandwidth.
        Not with Bitflow! We'll download your files in the blink of an eye!`,
      img: '/static/imgs/features/bitflow_speed.svg',
      name: 'Download big files faster',
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>
