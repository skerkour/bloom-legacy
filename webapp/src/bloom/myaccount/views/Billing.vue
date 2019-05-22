<template>
<div>
  <v-container>
    <v-layout row wrap justify-left>

      <v-flex xs12 v-if="is_loading" justify-center>
        <v-card class="elevation-0">
          <v-layout row wrap justify-center>
            <v-progress-circular
              :size="50"
              color="primary"
              indeterminate
            ></v-progress-circular>
          </v-layout>
        </v-card>
      </v-flex>

      <v-flex xs12 text-xs-center v-if="error">
        <v-alert icon="mdi-alert-circle" :value="error" type="error">
          {{ error }}
        </v-alert>
      </v-flex>

      <v-flex xs12>
        <v-card class="elevation-0">
          <v-card-title>
            <div class="headline text-xs-left">
              Payment methods
            </div>
          </v-card-title>
          <v-card-text>
              <v-data-table
                :headers="headers"
                :items="payment_methods"
                hide-actions
              >
                <template slot="no-data">
                  <p class="text-xs-center">
                    No payment method yet
                  </p>
                </template>
                <template v-slot:items="props">
                  <td>{{ props.item.name }}</td>
                  <td class="text-xs-right">{{ props.item.calories }}</td>
                  <td class="text-xs-right">{{ props.item.fat }}</td>
                  <td class="text-xs-right">{{ props.item.carbs }}</td>
                  <td class="text-xs-right">{{ props.item.protein }}</td>
                  <td class="text-xs-right">{{ props.item.iron }}</td>
                </template>
              </v-data-table>

          </v-card-text>
        </v-card>
      </v-flex>
      <v-flex class="mb-3 mt-3" xs12>
        <v-btn color="primary" @click="open_add_payment_dialog" :loading="is_loading">
          Add payment method
        </v-btn>
      </v-flex>
      <v-flex xs12>
        <v-divider></v-divider>
      </v-flex>




      <v-flex xs12>
        <v-card class="elevation-0">
          <v-card-title>
            <div class="headline text-xs-left">
              Subscriptions
            </div>
          </v-card-title>
          <v-container grid-list-xl text-xs-left>
          </v-container>
        </v-card>
      </v-flex>
      <v-flex xs12>
        <v-divider></v-divider>
      </v-flex>




      <v-flex xs12>
        <v-card class="elevation-0">
          <v-card-title>
            <div class="headline text-xs-left">
              Invoices
            </div>
          </v-card-title>
          <v-container grid-list-xl text-xs-left>
          </v-container>
        </v-card>
      </v-flex>


    </v-layout>
  </v-container>


  <blm-billing-add-payment-method-dialog
  :visible="display_add_payment_dialog"
  @close="display_add_payment_dialog = false"
  @add="payment_method_added" />
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import AddPaymentMethodDialog from '../components/AddPaymentMethodDialog.vue';

@Component({
  components: {
    'blm-billing-add-payment-method-dialog': AddPaymentMethodDialog,
  },
})
export default class Devices extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  display_add_payment_dialog = false;
  payment_methods: any[] = [];
  invoices: any[] = [];
  subscriptions: any[] = [];
  headers = [
    {
      sortable: false,
      text: 'Card',
    },
    {
      sortable: false,
      text: 'Date Added',
    },
    {
      sortable: false,
      text: 'Default',
    },
    { text: 'Actions', sortable: false },
  ];


  // computed
  // lifecycle
  // watch
  // methods
  open_add_payment_dialog() {
    this.display_add_payment_dialog = true;
  }

  payment_method_added() {

  }
}
</script>


<style scoped lang="scss">
</style>
