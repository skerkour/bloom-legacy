<template>
  <v-dialog
    v-model="show"
    max-width="400px"
    @keydown.esc="show = false"
  >
    <v-card>
      <v-card-title class="blue darken-1 white--text headline">
        New payment method
      </v-card-title>

      <v-card-text>
        <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
          {{ error }}
        </v-alert>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn flat @click="show = false" :disabled="is_loading">Cancel</v-btn>
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


@Component({})
export default class AddPaymentMethodDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;


  // data
  error = '';
  is_loading = false;


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
  }
}
</script>
