<template>
  <v-container>
    <v-row justify="center">
      <v-col cols="12">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row class="text-left" v-if="group">
      <v-col cols="12">
        <h1>{{ group.name }}</h1>
        <v-subheader>{{ group.id }}</v-subheader>
      </v-col>

      <v-col cols="12">
        <h1>Info</h1>
      </v-col>

      <v-col cols="6">
        <v-text-field label="Name" disabled :value="group.name" />
      </v-col>

      <v-col cols="12">
        <v-textarea disabled label="Description" :vlaue="group.description" />
      </v-col>


      <v-col cols="12">
        <h1>Billing</h1>
      </v-col>

      <v-col cols="12">
         <h2 class="headline">Plan</h2>
         <p>{{ group.subscription.plan.product }}</p>
      </v-col>

      <v-col cols="12">
        <h2 class="headline">Invoices</h2>
      </v-col>
      <v-col cols="12">
        <blm-billing-table-invoices :loading="loading" :invoices="invoices" />
      </v-col>

      <v-col cols="12">
        <h2 class="headline">Payment methods</h2>
      </v-col>
      <v-col cols="12">
        <blm-billing-table-payment-methods :loading="loading" :payment-methods="paymentMethods" />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import {
  Group, Invoice, PaymentMethod,
} from '@/api/models';
import { Method } from '@/core/groups';
import { FetchGroupDetailsParams } from '@/core/messages';
import core from '@/core';
import PaymentMethodsTable from '@/ui/billing/components/payment_methods_table.vue';
import InvoicesTable from '@/ui/billing/components/invoices_table.vue';


@Component({
  components: {
    'blm-billing-table-payment-methods': PaymentMethodsTable,
    'blm-billing-table-invoices': InvoicesTable,
  },
})
export default class AdminGroupView extends Vue {
  // props
  // data
  error = '';
  group: Group | null = null;
  loading = false;
  groupId = '';

  // computed
  get invoices(): Invoice[] {
    if (this.group === null) {
      return [];
    }
    return this.group.invoices!.nodes;
  }

  get paymentMethods(): PaymentMethod[] {
    if (this.group === null) {
      return [];
    }
    return this.group.paymentMethods!.nodes;
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
    const params: FetchGroupDetailsParams = {
      groupID: this.groupId,
    };

    try {
      this.group = await core.call(Method.FetchGroupDetails, params);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
