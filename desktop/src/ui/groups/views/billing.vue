<template>
  <v-container fluid class="text-left">
    <h1>Billing</h1>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class GroupsBillingView extends Vue {
  // props
  // data
  loading = false;
  error = '';
  groupId = '';

  // computed
  created() {
    this.groupId = this.$route.params.groupId;
  }

  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
