<template>
  <blm-feature-page
    name="Arcade"
    description="The best place to play"
    hero="/static/imgs/features/arcade_hero.svg"
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
      description: `Gaming should not be frustrating because the lack of a specific device. Bloom
      Arcade's games can run on any platform with a web browser`,
      img: '/static/imgs/features/arcade_devices.svg',
      name: 'Play from any device',
    },
    {
      description: 'None of our games contains ad.',
      img: '/static/imgs/features/arcade_privacy.svg',
      name: 'Committed to your privacy',
    },
    // {
    //   description: 'We are committed to provide the best multiplayer experience.',
    //   img: '/static/imgs/features/arcade_friends.svg',
    //   name: 'Never stop the fun with friends',
    // },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>
