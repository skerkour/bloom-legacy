<template>
  <v-container>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <blm-admin-table-plans :loading="isLoading" :plans="plans"
      @deleted="onPlanDeleted" @updated="onPlanUpdated" @created="onPlanCreated" />

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmAdminTablePlans from '../components/plans_table.vue';
import * as models from '@/api/models';
import core from '@/core';
import { Method as BillingMethod } from '@/core/billing';


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
      const res = await core.call(BillingMethod.FetchPlans, core.Empty);
      this.plans = res.edges.map((edge: models.Maybe<models.BillingPlanEdge>) => edge!.node!);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  onPlanDeleted(deletedPlan: models.BillingPlan) {
    this.plans = this.plans.filter((plan: models.BillingPlan) => plan.id !== deletedPlan.id);
  }

  onPlanUpdated(updatedPlan: models.BillingPlan) {
    const pos = this.plans.map((plan: models.BillingPlan) => plan.id).indexOf(updatedPlan.id);
    this.plans.splice(pos, 1);
    this.plans = [updatedPlan, ...this.plans];
  }

  onPlanCreated(newPlan: models.BillingPlan) {
    this.plans.push(newPlan);
  }
}
</script>


<style lang="scss" scoped>
</style>
