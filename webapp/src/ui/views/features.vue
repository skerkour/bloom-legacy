<template>
  <v-container fluid>
    <b-features  class="mt-5" />
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BFeatures from '@/ui/components/kernel/features.vue';

export default VueApp.extend({
  name: 'BFeaturesView',
  components: {
    BFeatures,
  },
});
</script>


<style lang="scss" scoped>
</style>
