<template>
  <blm-feature-page
    name="Arcade"
    :description="$t('features.arcade.description')"
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
export default class ArcadeView extends Vue {
  // props
  // data
  // computed
  get features(): any[] {
    return [
      {
        description: this.$t('features.arcade.feature1.description'),
        img: '/static/imgs/features/arcade_devices.svg',
        name: this.$t('features.arcade.feature1.name'),
      },
      {
        description: this.$t('features.arcade.feature2.description'),
        img: '/static/imgs/features/arcade_privacy.svg',
        name: this.$t('features.arcade.feature2.name'),
      },
      // {
      //   description: 'We are committed to provide the best multiplayer experience.',
      //   img: '/static/imgs/features/arcade_friends.svg',
      //   name: 'Never stop the fun with friends',
      // },
    ];
  }
  // lifecycle
  // watch
  // methods
}
</script>
