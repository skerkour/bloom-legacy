<template>
  <blm-feature-page
    name="Notes"
    description="Quickly capture what’s on your mind."
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
      description: `Bloom Notes helps you capture and prioritize ideas, projects,
            so nothing falls through the cracks.`,
      img: '/static/imgs/features/notes_organized.svg',
      name: 'Feel organized without the effort',
    },
    {
      description: `Take note. It's easy to pull up your content from anywhere. Start on your
        laptop then update notes on your phone. Bloom Notes works on any device or platform.`,
      img: '/static/imgs/features/notes_devices.svg',
      name: 'Access from anywhere',
    },
    // {
    //   description: 'Quickly filter and search for notes. Find what you\'re looking \
    //     for even faster, and let Bloom notes do the remembering for you.',
    //   logo: '/static/imgs/features/notes_search.svg',
    //   name: 'Find what you need, fast ',
    // },
  ];

  // computed
  // lifecycle
  // watch
  // methods
}
</script>
