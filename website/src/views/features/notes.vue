<template>
  <blm-feature-page
    name="Notes"
    :description="this.$t('features.notes.description')"
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
export default class NotesView extends Vue {
  // props
  // data
  // computed
  get features(): any[] {
    return [
      {
        description: this.$t('features.notes.feature1.description'),
        img: '/static/imgs/features/notes_organized.svg',
        name: this.$t('features.notes.feature1.name'),
      },
      {
        description: this.$t('features.notes.feature2.description'),
        img: '/static/imgs/features/notes_devices.svg',
        name: this.$t('features.notes.feature2.name'),
      },
      // {
      //   description: 'Quickly filter and search for notes. Find what you\'re looking \
      //     for even faster, and let Bloom notes do the remembering for you.',
      //   logo: '/static/imgs/features/notes_search.svg',
      //   name: 'Find what you need, fast ',
      // },
    ];
  }
  // lifecycle
  // watch
  // methods
}
</script>
