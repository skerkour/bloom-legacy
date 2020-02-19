<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close()"
    @click:outside="close()"
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >
    <v-card>
      <v-card-title class="headline">
        Add a payment method
      </v-card-title>
      <v-card-text>
        <form>
          <v-container>
            <v-row>
              <v-col cols="12">
                <v-text-field label="Card number" outlined v-model="number" v-mask="numberMask"
                  placeholder="4242 4242 4242 4242" required/>
              </v-col>
            </v-row>
            <v-row>
              <v-col cols="4">
                <v-text-field label="Expiration month" outlined v-model="expMonth"
                  v-mask="expMonthMask" placeholder="02" required/>
              </v-col>
              <v-col cols="4">
                <v-text-field label="Expiration year" outlined v-model="expYear" placeholder="2021"
                  v-mask="expYearMask" required />
              </v-col>
              <v-col cols="4">
                <v-text-field label="CVC" outlined v-model="cvc"
                  v-mask="cvcMask" placeholder="314" required />
              </v-col>
            </v-row>
          </v-container>
        </form>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
          <v-btn text @click="close()">
            Cancel
          </v-btn>
          <v-btn color="primary" @click="add()">
            Add
          </v-btn>

      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import { mask } from 'vue-the-mask';
import { NewStripeCard } from '@/core/billing';

@Component({
  directives: {
    mask,
  },
})
export default class AddPaymentMethodDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;

  // data
  number = '';
  cvc = '';
  expMonth = '';
  expYear = '';
  numberMask ='#### #### #### ####';
  cvcMask = '###';
  expMonthMask = '##';
  expYearMask = '####';

  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('closed');
    }
  }

  // lifecycle
  // watch
  // methods
  close() {
    this.number = '';
    this.cvc = '';
    this.expMonth = '';
    this.expYear = '';
    this.show = false;
  }

  add() {
    const card: NewStripeCard = {
      number: this.number,
      cvc: this.cvc,
      exp_month: this.expMonth,
      exp_year: this.expYear,
    };
    this.$emit('added', card);
    this.close();
  }
}
</script>


<style lang="scss" scoped>
</style>
