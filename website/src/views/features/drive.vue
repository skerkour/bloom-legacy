<template>
  <blm-feature-page
    name="Drive"
    description="A safe place for all your files"
    hero="/static/imgs/features/drive_hero.svg"
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
      description: `Keep photos, stories, designs, drawings, recordings, videos, and more.
        Your first <b>3 GB</b> of storage are free.`,
      img: '/static/imgs/features/drive_files.svg',
      name: 'Store any file',
    },
    {
      description: `Your files in Drive can be reached from any smartphone, tablet, or computer.
        So wherever you go, your files follow.`,
      img: '/static/imgs/features/drive_devices.svg',
      name: 'See your stuff anywhere',
    },
    {
      description: `If something happens to your device, you don't have to worry about losing
        your files or photos – they're in your Bloom Drive.`,
      img: '/static/imgs/features/drive_security.svg',
      name: 'Keep your files safe',
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>
