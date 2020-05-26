<template>
  <v-container fluid style="height: calc(100vh)" class="text-left overflow-y-auto">

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

    <v-row v-if="me">
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
                me.subscription.plan.product }">
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
                  <v-btn color="primary" v-if="plan.product !== me.subscription.plan.product"
                      @click="changePlan(plan)" :loading="loading">
                    <span v-if="plan.price > me.subscription.plan.price">Upgrade</span>
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
import * as models from '@/api/models';
import core from '@/core';
import { Method, NewStripeCard } from '@/core/billing';


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
  plans: models.BillingPlan[] = [];
  me: models.User | null = null;
  planAfterAddingPaymentMethod: models.BillingPlan | null = null;
  showAddPaymentDialog = false;
  stripePublicKey: string | null = null;
  paymentMethodError = '';

  // computed
  get invoices(): models.Invoice[] {
    if (this.me === null) {
      return [];
    }
    return this.me.invoices!.nodes;
  }

  get paymentMethods(): models.PaymentMethod[] {
    if (this.me === null) {
      return [];
    }
    return this.me.paymentMethods!.nodes;
  }

  // lifecycle
  created() {
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.loading = true;

    try {
      const res = await core.call(Method.FetchMyProfile, core.Empty);
      this.me = res.me;
      this.plans = res.billingPlans
        .nodes.map((node: models.Maybe<models.BillingPlan>) => node!);
      this.stripePublicKey = res.stripePublicKey;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async changePlan(newPlan: models.BillingPlan) {
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
      stripePublicKey: this.stripePublicKey,
      card,
    };

    try {
      const res: models.PaymentMethod = await core
        .call(Method.AddPaymentMethod, params);
      this.me!.paymentMethods!.nodes.push(res);
      if (this.planAfterAddingPaymentMethod) {
        await this.updateSubscription(this.planAfterAddingPaymentMethod);
      }
    } catch (err) {
      this.paymentMethodError = err.message;
    } finally {
      this.loading = false;
    }
  }

  async removePaymentMethod(paymentMenthod: models.PaymentMethod) {
    this.paymentMethodError = '';
    this.loading = true;
    const input: models.RemovePaymentMethodInput = {
      id: paymentMenthod.id,
    };

    try {
      await core.call(Method.RemovePaymentMethod, input);
      this.me!.paymentMethods!.nodes = this.me!.paymentMethods!.nodes
        .filter((node: models.Maybe<models.PaymentMethod>) => node!.id !== paymentMenthod.id); // eslint-disable-line
    } catch (err) {
      this.paymentMethodError = err.message;
    } finally {
      this.loading = false;
    }
  }

  async updateSubscription(newPlan: models.BillingPlan) {
    this.error = '';
    this.loading = true;
    this.planAfterAddingPaymentMethod = null;
    const input: models.UpdateBillingSubscriptionInput = {
      planId: newPlan.id,
    };

    try {
      const res = await core.call(Method.UpdateSubscription, input);
      this.me!.subscription = res;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async changeDefaultPaymentMethod(newDefaultPaymentMethod: models.PaymentMethod) {
    this.error = '';
    this.loading = true;
    const input: models.ChangeDefaultPaymentMethodInput = {
      id: newDefaultPaymentMethod.id,
    };

    try {
      await core.call(Method.ChangeDefaultPaymentMethod, input);
      const paymentMehtods = this.me!.paymentMethods!
        .nodes.map((paymentMethod: models.Maybe<models.PaymentMethod>, index: number) => {
          if (paymentMethod!.isDefault) {
            const newPaymentMethod = paymentMethod;
            newPaymentMethod!.isDefault = false;
            this.$set(this.me!.paymentMethods!.nodes, index, newPaymentMethod);
          }
          if (paymentMethod!.id === newDefaultPaymentMethod.id) {
            const newPaymentMethod = paymentMethod;
            newPaymentMethod!.isDefault = true;
            this.$set(this.me!.paymentMethods!.nodes, index, newPaymentMethod);
          }
          return paymentMethod!;
        });
      this.me!.paymentMethods!.nodes = paymentMehtods;
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
