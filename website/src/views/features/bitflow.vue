<template>
  <blm-feature-page
    name="Bitflow"
    :description="$t('features.bitflow.description')"
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
  // computed
  get features(): any[] {
    return [
      {
        description: this.$t('features.bitflow.feature1.description'),
        img: '/static/imgs/features/bitflow_devices.svg',
        name: this.$t('features.bitflow.feature1.name'),
      },
      {
        description: this.$t('features.bitflow.feature2.description'),
        img: '/static/imgs/features/bitflow_protocols.svg',
        name: this.$t('features.bitflow.feature2.name'),
      },
      {
        description: this.$t('features.bitflow.feature3.description'),
        img: '/static/imgs/features/bitflow_speed.svg',
        name: this.$t('features.bitflow.feature3.name'),
      },
    ];
  }
  // lifecycle
  // watch
  // methods
}
</script>
