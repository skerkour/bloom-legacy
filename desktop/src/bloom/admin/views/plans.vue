<template>
  <v-container>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <blm-admin-table-plans :loading="isLoading" :plans="plans" />

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmAdminTablePlans from '../components/plans_table.vue';
// import InvoicesTable from '../components/invoices_table.vue';
import * as models from '@/api/models';
import core from '@/core';
import AdminMethods from '@/bloom/admin/core/methods';


@Component({
  components: {
    BlmAdminTablePlans,
  },
})
export default class Plans extends Vue {
  // props
  // data
  error = '';
  isLoading = false;
  plans: models.BillingPlan[] = [];

  // computed
  // lifecycle
  created() {
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.isLoading = true;

    try {
      this.plans = await core.call(AdminMethods.FetcBillinghPlans, core.Empty);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
