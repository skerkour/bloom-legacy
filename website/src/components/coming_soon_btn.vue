<template>
  <v-btn x-large color="primary" class="mt-5 text-left" :outlined="outlined" to="/download">
    Coming January 2020
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
