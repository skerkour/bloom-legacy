<template>
  <div>
    <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
      {{ error }}
    </v-alert>

  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class Account extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  account: any = null;
  selected: any[] = [];

  // computed
  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data(loading?: boolean) {
    if (loading !== false) {
      this.error = '';
      this.is_loading = true;
    }
    try {
      this.account = await api.get(`${api.ADMIN}/v1/accounts/${this.$route.params.account_id}`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
</style>
