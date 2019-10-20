<template>
  <v-footer absolute class="font-weight-medium" color="transparent">
    <v-col class="text-center" cols="12">
      {{ new Date().getFullYear() }} —
      <strong>Bloom</strong>
    </v-col>
  </v-footer>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Footer extends Vue {}
</script>
