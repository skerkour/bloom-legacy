<template>
  <v-container fluid class="text-left">

    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-row v-if="isLoading" class="justify-center text-center">
      <v-col cols="12">
        <v-progress-circular
          indeterminate
          color="primary"
          size="50"
        />
      </v-col>
    </v-row>

    <v-row v-if="me">
      <v-col xs="12" lg="10" xl="8">

        <v-row>
          <v-col cols="12">
            <div class="headline">
              Plans
            </div>
          </v-col>
        </v-row>

        <v-row justify="space-around" align="stretch">
          <v-col cols="12" md="3" class="text-center mt-5" align-self="stretch"
            v-for="plan in plans" :key="plan.id">
            <v-hover v-slot:default="{ hover }">
              <v-card class="mx-auto blm-pricing-card" outlined :elevation="hover ? 4 : 0"
                :class="{ 'on-hover': hover, 'blm-billing-myplan': plan.product ===
                me.subscription.plan.product }">
                <v-card-title class="display-1 justify-center">{{ plan.name }}</v-card-title>
                <div class="v-card--plan__price pa-5 col col-12" v-if="plan.price === 0">
                  <div class="d-inline-block">
                    <span class="display-3 font-weight-bold">Free</span>
                  </div>
                </div>
                <div class="v-card--plan__price pa-5 col col-12" v-else>
                  <div class="d-inline-block">
                    <span class="display-3 font-weight-bold">{{ plan.price }}€</span>
                  </div>
                  <span class="caption"> /mo </span>
                </div>
                <v-card-text class="blm-pricing-card-text" v-html="plan.description">
                </v-card-text>
              </v-card>
            </v-hover>
          </v-col>
        </v-row>


        <v-row>
          <v-col cols="12">
            <div class="headline">
              Payment methods
            </div>
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12">
            <blm-myaccount-table-payment-methods :loading="isLoading" :methods="paymentMethods"/>
          </v-col>
        </v-row>


        <v-row>
          <v-col cols="12">
            <div class="headline">
              Invoices
            </div>
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12">
            <blm-myaccount-table-invoices :loading="isLoading" :invoices="invoices" />
          </v-col>
        </v-row>


      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import PaymentMethodsTable from '../components/payment_methods_table.vue';
import InvoicesTable from '../components/invoices_table.vue';
import * as models from '@/api/models';
import core from '@/core';
import { Method } from '@/core/billing';


@Component({
  components: {
    'blm-myaccount-table-payment-methods': PaymentMethodsTable,
    'blm-myaccount-table-invoices': InvoicesTable,
  },
})
export default class Billing extends Vue {
  // props
  // data
  error = '';
  isLoading = false;
  plans: models.BillingPlan[] = [];
  me: models.User | null = null;

  // computed
  get invoices(): models.Invoice[] {
    if (this.me === null) {
      return [];
    }
    return this.me.invoices!.edges!.map((edge: models.Maybe<models.InvoiceEdge>) => edge!.node!);
  }

  get paymentMethods(): models.PaymentMethod[] {
    if (this.me === null) {
      return [];
    }
    return this.me.paymentMethods!
      .edges!.map((edge: models.Maybe<models.PaymentMethodEdge>) => edge!.node!);
  }

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
      const res = await core.call(Method.FetchMyProfile, core.Empty);
      this.me = res.me;
      this.plans = res.billingPlans
        .edges!.map((edge: models.Maybe<models.BillingPlanEdge>) => edge!.node!);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }
}
</script>


<style lang="scss">
.blm-pricing-card-text ul {
  list-style-type: none;
}

.blm-pricing-card-text ul li {
  margin-top: 9px;
}

.blm-billing-myplan {
  border: thin solid green !important;
}
</style>

<style scoped lang="scss">
.v-card__text {
  font-size: 20px !important;
}

.v-application p {
  margin-bottom: 0px;
}

.blm-pricing-card {
  height: 100%;
}
</style>
