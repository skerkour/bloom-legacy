<template>
  <v-container fluid class="text-left">

    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-row v-if="loading" class="justify-center text-center">
      <v-col cols="12">
        <v-progress-circular
          indeterminate
          color="primary"
          size="50"
        />
      </v-col>
    </v-row>

    <v-row v-if="group">
      <v-col xs="12" lg="10" xl="8">

        <v-row>
          <v-col cols="12">
            <div class="headline">
              Subscription
            </div>
          </v-col>
        </v-row>

        <v-row justify="space-around" align="stretch">
          <v-col cols="12" md="3" sm="6" class="text-center mt-5" align-self="stretch"
            v-for="plan in plans" :key="plan.id">
            <v-hover v-slot:default="{ hover }">
              <v-card class="mx-auto blm-pricing-card" outlined :elevation="hover ? 4 : 0"
                :class="{ 'on-hover': hover, 'blm-billing-myplan': plan.product ===
                group.subscription.plan.product }">
                <v-card-title class="display-1 justify-center">{{ plan.name }}</v-card-title>
                <div class="v-card--plan__price pa-5 col col-12" v-if="plan.price === 0">
                  <div class="d-inline-block">
                    <span class="display-3 font-weight-bold">Free</span>
                  </div>
                </div>
                <div class="v-card--plan__price pa-5 col col-12" v-else>
                  <div class="d-inline-block">
                    <span class="display-3 font-weight-bold">{{ plan.price / 100 }}€</span>
                  </div>
                  <span class="caption"> /mo </span>
                </div>
                <v-card-text class="blm-pricing-card-text" v-html="plan.description">
                </v-card-text>
                <v-card-actions class="justify-center blm-pricing-card-actions text-center pb-3">
                  <v-btn color="primary" v-if="plan.product !== group.subscription.plan.product"
                      @click="changePlan(plan)" :loading="loading">
                    <span v-if="plan.price > group.subscription.plan.price">Upgrade</span>
                    <span v-else>Downgrade</span>
                  </v-btn>
                  <v-btn outlined color="success" v-else>
                    Current plan
                  </v-btn>
                </v-card-actions>
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
            <v-alert icon="mdi-alert-circle" type="error" :value="paymentMethodError !== ''">
              {{ paymentMethodError }}
            </v-alert>
            <blm-billing-table-payment-methods :loading="loading"
              :payment-methods="paymentMethods" @removed="removePaymentMethod"
              @changed="changeDefaultPaymentMethod" />
          </v-col>
          <v-col class="d-flex justify-space-around">
            <v-btn color="primary" class="new-btn" @click="openAddPaymentMethodDialog"
                :loading="loading">
              Add <v-icon right>mdi-plus</v-icon>
            </v-btn>
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
            <blm-billing-table-invoices :loading="loading" :invoices="invoices" />
          </v-col>
        </v-row>


      </v-col>
    </v-row>

    <blm-billing-add-payment-method-dialog :visible="showAddPaymentDialog"
      @closed="addPaymentMethodDialogClosed" @added="addPaymentMethod" />
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import PaymentMethodsTable from '@/ui/billing/components/payment_methods_table.vue';
import InvoicesTable from '@/ui/billing/components/invoices_table.vue';
import AddPaymentMethodDialog from '@/ui/billing/components/add_payment_method_dialog.vue';
import core from '@/core';
import { Method, NewStripeCard, FetchGroupProfileParams } from '@/core/billing';
import {
  Group, BillingPlan, Invoice, PaymentMethod, InvoiceEdge, Maybe, PaymentMethodEdge,
  BillingPlanEdge, ChangeDefaultPaymentMethodInput, RemovePaymentMethodInput,
  UpdateBillingSubscriptionInput,
} from '@/api/models';


@Component({
  components: {
    'blm-billing-table-payment-methods': PaymentMethodsTable,
    'blm-billing-table-invoices': InvoicesTable,
    'blm-billing-add-payment-method-dialog': AddPaymentMethodDialog,
  },
})
export default class Billing extends Vue {
  // props
  // data
  error = '';
  loading = false;
  plans: BillingPlan[] = [];
  group: Group | null = null;
  planAfterAddingPaymentMethod: BillingPlan | null = null;
  showAddPaymentDialog = false;
  stripePublicKey: string | null = null;
  paymentMethodError = '';
  groupId = '';

  // computed
  get invoices(): Invoice[] {
    if (this.group === null) {
      return [];
    }
    return this.group.invoices!.edges!.map((edge: Maybe<InvoiceEdge>) => edge!.node!);
  }

  get paymentMethods(): PaymentMethod[] {
    if (this.group === null) {
      return [];
    }
    return this.group.paymentMethods!
      .edges!.map((edge: Maybe<PaymentMethodEdge>) => edge!.node!);
  }

  // lifecycle
  created() {
    this.groupId = this.$route.params.groupId;
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.loading = true;
    const params: FetchGroupProfileParams = {
      id: this.groupId,
    };

    try {
      const res = await core.call(Method.FetchGroupProfile, params);
      this.group = res.group;
      this.plans = res.billingPlans
        .edges!.map((edge: Maybe<BillingPlanEdge>) => edge!.node!);
      this.stripePublicKey = res.stripePublicKey;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async changePlan(newPlan: BillingPlan) {
    this.planAfterAddingPaymentMethod = null;
    if (this.paymentMethods.length === 0) {
      this.planAfterAddingPaymentMethod = newPlan;
      await this.openAddPaymentMethodDialog();
    } else {
      this.updateSubscription(newPlan);
    }
  }

  async openAddPaymentMethodDialog() {
    this.showAddPaymentDialog = true;
  }

  addPaymentMethodDialogClosed() {
    this.showAddPaymentDialog = false;
  }

  async addPaymentMethod(card: NewStripeCard) {
    this.paymentMethodError = '';
    this.loading = true;
    const params = {
      groupId: this.groupId,
      stripePublicKey: this.stripePublicKey,
      card,
    };

    try {
      const res: Maybe<PaymentMethod> = await core
        .call(Method.AddPaymentMethod, params);
      this.group!.paymentMethods!.edges!.push({ node: res, cursor: '' });
      if (this.planAfterAddingPaymentMethod) {
        await this.updateSubscription(this.planAfterAddingPaymentMethod);
      }
    } catch (err) {
      this.paymentMethodError = err.message;
    } finally {
      this.loading = false;
    }
  }

  async removePaymentMethod(paymentMenthod: PaymentMethod) {
    this.paymentMethodError = '';
    this.loading = true;
    const input: RemovePaymentMethodInput = {
      id: paymentMenthod.id,
    };

    try {
      await core.call(Method.RemovePaymentMethod, input);
      this.group!.paymentMethods!.edges = this.group!.paymentMethods!.edges!
        .filter((edge: Maybe<PaymentMethodEdge>) => edge!.node!.id !== paymentMenthod.id); // eslint-disable-line
    } catch (err) {
      this.paymentMethodError = err.message;
    } finally {
      this.loading = false;
    }
  }

  async updateSubscription(newPlan: BillingPlan) {
    this.error = '';
    this.loading = true;
    this.planAfterAddingPaymentMethod = null;
    const input: UpdateBillingSubscriptionInput = {
      groupId: this.groupId,
      planId: newPlan.id,
    };

    try {
      const res = await core.call(Method.UpdateSubscription, input);
      this.group!.subscription = res;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async changeDefaultPaymentMethod(newDefaultPaymentMethod: PaymentMethod) {
    this.error = '';
    this.loading = true;
    const input: ChangeDefaultPaymentMethodInput = {
      id: newDefaultPaymentMethod.id,
    };

    try {
      await core.call(Method.ChangeDefaultPaymentMethod, input);
      const paymentMehtods = this.group!.paymentMethods!
        .edges!.map((edge: Maybe<PaymentMethodEdge>, index: number) => {
          if (edge!.node!.isDefault) {
            const newEdge = edge;
            newEdge!.node!.isDefault = false;
            this.$set(this.group!.paymentMethods!.edges!, index, newEdge);
          }
          if (edge!.node!.id === newDefaultPaymentMethod.id) {
            const newEdge = edge;
            newEdge!.node!.isDefault = true;
            this.$set(this.group!.paymentMethods!.edges!, index, newEdge);
          }
          return edge;
        });
      this.group!.paymentMethods!.edges = paymentMehtods;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
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
  padding-bottom: 50px;
  position: relative;
}

.blm-pricing-card-actions {
  position: absolute;
  bottom: 0;
  width: 100%;
}
</style>
