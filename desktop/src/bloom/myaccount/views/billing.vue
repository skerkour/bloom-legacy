<template>
  <v-container fluid class="text-left">

    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-row>
      <v-col cols="12">
        <div class="headline">
          My Plan
        </div>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12">
        <!-- TODO: display all plans, with a button to change -->
      </v-col>
    </v-row>

    <v-row>
      <v-col><v-divider/></v-col>
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
        <blm-myaccount-table-payment-methods :loading="isLoading" :methods="methods" />
      </v-col>
    </v-row>

    <v-row>
      <v-col><v-divider/></v-col>
    </v-row>

    <v-row>
      <v-col><v-divider/></v-col>
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

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import PaymentMethodsTable from '../components/payment_methods_table.vue';
import InvoicesTable from '../components/invoices_table.vue';
import SubscriptionsTable from '../components/subscriptions_table.vue';

@Component({
  components: {
    'blm-myaccount-table-payment-methods': PaymentMethodsTable,
    'blm-myaccount-table-invoices': InvoicesTable,
    'blm-myaccount-table-subscriptions': SubscriptionsTable,
  },
})
export default class Billing extends Vue {
  // props
  // data
  error = '';
  isLoading = false;
  methods = [];
  invoices = [];
  subscriptions = [];

  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
