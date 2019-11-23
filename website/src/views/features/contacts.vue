<template>
  <blm-feature-page
    name="Contacts"
    description="Keep your contacts organized and up to date"
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
  features = [
    {
      description: `Safely back up your contacts to the cloud
            to access from any device.`,
      img: '/static/imgs/features/contacts_devices.svg',
      name: 'Back up your contacts and sync them across all your devices',
    },
    {
      description: `Easily add contacts and edit information like phone numbers, emails,
        and profiles.`,
      img: '/static/imgs/features/contacts_search.svg',
      name: 'Keep your contacts organized and up to date',
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>
