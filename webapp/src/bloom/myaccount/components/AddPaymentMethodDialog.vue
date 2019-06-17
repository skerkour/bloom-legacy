<template>
  <v-dialog
    v-model="show"
    max-width="800px"
    @keydown.esc="show = false"
  >
    <v-card>
      <v-card-title class="blue darken-1 white--text headline">
        New payment method
      </v-card-title>

      <v-card-text>
        <v-container grid-list-md>
        <v-layout row wrap>
          <v-flex xs12>
            <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
            {{ error }}
            </v-alert>
          </v-flex>

          <v-flex xs12>
            <h3>
              Card details
            </h3>
          </v-flex>

          <v-flex xs12 sm6>
            <v-text-field
              label="Card Number"
              requiredv-model="card_number"
              autocomplete="off"
              required
              hint="1234-1234-1234-1234"
              persistent-hint
              mask="credit-card"
            ></v-text-field>
          </v-flex>
          <v-flex xs12 sm3>
            <v-text-field
            label="Expiration Date"
            required
            v-model="card_expiration_date"
            autocomplete="off"
            hint="MM/YY"
            persistent-hint
            mask="##/##"
            ></v-text-field>
          </v-flex>
          <v-flex xs12 sm3 class="mb-4">
            <v-text-field
            label="CVC"
            required
            v-model="card_cvc"
            autocomplete="off"
            hint="123"
            persistent-hint
            mask="###"
            ></v-text-field>
          </v-flex>

          <v-flex xs12>
            <h3>
              Billing Address
            </h3>
          </v-flex>
          <v-flex xs12 sm6>
            <v-text-field
            label="First Name"
            required
            v-model="first_name"
            autocomplete="off"
            ></v-text-field>
          </v-flex>
            <v-flex xs12 sm6>
              <v-text-field
              label="Last Name"
              required
              v-model="last_name"
              autocomplete="off"
              ></v-text-field>
            </v-flex>

            <v-flex xs12 sm8>
              <v-text-field
              label="Street Address"
              required
              v-model="street_address"
              autocomplete="off"
              ></v-text-field>
            </v-flex>
            <v-flex xs12 sm4>
              <v-text-field
              label="City"
              required
              v-model="city"
              autocomplete="off"
              ></v-text-field>
            </v-flex>
            <v-flex xs12 sm4>
              <v-text-field
              label="Sate / Region"
              v-model="state"
              autocomplete="off"
              ></v-text-field>
            </v-flex>
            <v-flex xs12 sm4>
              <v-text-field
              label="Postal Code"
              required
              v-model="postal_code"
              autocomplete="off"
              ></v-text-field>
            </v-flex>
            <v-flex xs12 sm4>
              <v-text-field
              label="Country"
              required
              v-model="country"
              autocomplete="off"
              ></v-text-field>
            </v-flex>

        </v-layout>
        </v-container>
      </v-card-text>

      <v-card-actions>
        <a href="https://stripe.com" target="_blank" rel="noopener" class="mr-2">
        <img alt="powered by stripe" src="/kernel/static/imgs/powered_stripe.png" height="30"/>
        </a>
        <span>We never store your financial data.</span>
        <v-spacer></v-spacer>
        <v-btn flat @click="cancel" :disabled="is_loading">Cancel</v-btn>
        <v-btn
        @click="add_payment_method"
        color="primary"
        :loading="is_loading"
        >
          Add
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
const { log } = require('@bloom42/astro');
import axios from 'axios';
import config from '@/config';

@Component({})
export default class AddPaymentMethodDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;


  // data
  error = '';
  is_loading = false;

  card_number = '';
  card_expiration_date = '';
  card_cvc = '';
  first_name = '';
  last_name = '';
  street_address = '';
  city = '';
  state = '';
  postal_code = '';
  country = '';


  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('close');
    }
  }


  // lifecycle
  // watch
  // methods
  async add_payment_method() {
    // TODO: validate
    const payload = {
      card: {
        address_city: this.city,
        address_country: this.country,
        address_line1: this.street_address,
        address_zip: this.postal_code,
        cvc: this.card_cvc,
        exp_month: this.card_expiration_date.substr(0, 2),
        exp_year: this.card_expiration_date.substr(2, 2),
        name: `${this.first_name} ${this.last_name}`,
        number: this.card_number,
      },
    };
    log.debug(payload);
    try {
      const res = await axios.post('https://api.stripe.com/v1/tokens', payload, {
        auth: {
          password: '',
          username: config.STRIPE_PUBLIC_KEY,
        },
      });
      log.debug(res);
    } catch (err) {
      log.error(err);
    }
    this.cancel();
  }

  cancel() {
    this.card_expiration_date = '';
    this.card_number = '';
    this.card_cvc = '';
    this.first_name = '';
    this.last_name = '';
    this.street_address = '';
    this.city = '';
    this.state = '';
    this.postal_code = '';
    this.country = '';
    this.show = false;
  }
}
</script>
