<template>
  <blm-feature-page
    name="Drive"
    :description="$t('features.drive.description')"
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
  // computed
  get features(): any[] {
    return [
      {
        description: this.$t('features.drive.feature1.description'),
        img: '/static/imgs/features/drive_files.svg',
        name: this.$t('features.drive.feature1.name'),
      },
      {
        description: this.$t('features.drive.feature2.description'),
        img: '/static/imgs/features/drive_devices.svg',
        name: this.$t('features.drive.feature2.name'),
      },
      {
        description: this.$t('features.drive.feature3.description'),
        img: '/static/imgs/features/drive_security.svg',
        name: this.$t('features.drive.feature3.name'),
      },
    ];
  }

  // lifecycle
  // watch
  // methods
}
</script>
