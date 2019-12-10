<template>
  <blm-feature-page
    name="Phaser"
    description="The cloud-native security scanner"
    hero="/static/imgs/features/notes_hero.svg"
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
      description: 'TODO',
      img: '/static/imgs/features/notes_organized.svg',
      name: 'TODO',
    },
  ];

  // computed
  // lifecycle
  // watch
  // methods
}
</script>
