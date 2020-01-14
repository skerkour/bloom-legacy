<template>
  <blm-feature-page
    name="Contacts"
    :description="$t('features.contacts.description')"
    hero="/static/imgs/features/contacts_hero.svg"
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
        description: this.$t('features.contacts.feature1.description'),
        img: '/static/imgs/features/contacts_devices.svg',
        name: this.$t('features.contacts.feature1.name'),
      },
      {
        description: this.$t('features.contacts.feature2.description'),
        img: '/static/imgs/features/contacts_search.svg',
        name: this.$t('features.contacts.feature2.name'),
      },
    ];
  }
  // lifecycle
  // watch
  // methods
}
</script>
