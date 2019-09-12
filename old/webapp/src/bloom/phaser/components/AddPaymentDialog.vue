<template>
  <v-dialog
  v-model="show"
  max-width="400px"
  @keydown.esc="show = false"
  >
    <v-card>
      <v-card-title class="blue darken-1 white--text headline">
        Limit reached
      </v-card-title>

      <v-card-text>
        <v-container>
          <v-layout row wrap>
            <v-flex xs12>
              <p class="subheading">
                Please add a payment method and enable billing to create more scans
              </p>
              <div class="text-xs-center mt-2">
                <v-btn to="/myaccount/billing" color="primary">Go to Billing</v-btn>
              </div>
            </v-flex>
          </v-layout>
        </v-container>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn flat @click="close">Close</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';

@Component
export default class NewScanDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;


  // data
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
  close() {
    this.show = false;
  }
}
</script>
