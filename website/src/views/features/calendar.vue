<template>
  <blm-feature-page
    name="Calendar"
    :description="$t('features.calendar.description')"
    hero="/static/imgs/features/calendar_hero.svg"
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
        description: this.$t('features.calendar.feature1.description'),
        img: '/static/imgs/features/calendar_easy.svg',
        name: this.$t('features.calendar.feature1.name'),
      },
      {
        description: this.$t('features.calendar.feature2.description'),
        img: '/static/imgs/features/calendar_group.svg',
        name: this.$t('features.calendar.feature2.name'),
      },
    ];
  }
  // lifecycle
  // watch
  // methods
}
</script>
