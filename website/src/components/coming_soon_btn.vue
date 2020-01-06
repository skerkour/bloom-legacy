<template>
  <v-btn x-large color="primary" class="mt-5" :outlined="outlined" to="/download">
    <v-icon left v-text="icon" v-if="icon !== ''"/>
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
