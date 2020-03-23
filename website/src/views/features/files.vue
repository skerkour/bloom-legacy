<template>
  <blm-feature-page
    name="Files"
    :description="$t('features.files.description')"
    hero="/static/imgs/features/files_hero.svg"
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
export default class FilesView extends Vue {
  // props
  // data
  // computed
  get features(): any[] {
    return [
      {
        description: this.$t('features.files.feature1.description'),
        img: '/static/imgs/features/files_files.svg',
        name: this.$t('features.files.feature1.name'),
      },
      {
        description: this.$t('features.files.feature2.description'),
        img: '/static/imgs/features/files_devices.svg',
        name: this.$t('features.files.feature2.name'),
      },
      {
        description: this.$t('features.files.feature3.description'),
        img: '/static/imgs/features/files_security.svg',
        name: this.$t('features.files.feature3.name'),
      },
    ];
  }

  // lifecycle
  // watch
  // methods
}
</script>
