<template>
  <v-btn x-large color="primary" class="text-left" :outlined="outlined" to="/download">
    {{ $t('labels.coming_soon_btn') }}
  </v-btn>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class AppBar extends Vue {
  // props
  @Prop({ type: String }) platform?: string;
  @Prop({ type: Boolean }) outlined?: boolean;
}
</script>

<style>
</style>
