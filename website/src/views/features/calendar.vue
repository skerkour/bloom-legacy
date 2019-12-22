<template>
  <blm-feature-page
    name="Calendar"
    description="Make the most of every day"
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
  features = [
    {
      description: `Make the most of every day with a simple to use interface, designed to adapt to
      everyone's needs.`,
      img: '/static/imgs/features/calendar_easy.svg',
      name: 'Easy to use',
    },
    {
      description: `Share calendars with friends and coworkers and access them all from the same
      unified interface`,
      img: '/static/imgs/features/calendar_group.svg',
      name: 'All your calendars in one place',
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>
