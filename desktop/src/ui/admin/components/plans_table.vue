<template>
  <v-container fluid class="text-left">
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
      {{ error }}
    </v-alert>


    <v-toolbar flat dense>
      <v-spacer />
        <v-btn color="primary" class="new-btn" @click="newPlan">
          New <v-icon right>mdi-plus</v-icon>
        </v-btn>
    </v-toolbar>


    <v-data-table
      :headers="headers"
      :items="plans"
      item-key="id"
      :loading="isLoading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No Plans.
        </p>
      </template>

      <template v-slot:item="{ item }">
        <tr>
          <td>
            <span>{{ item.name }}</span>
          </td>
          <td>
            <span>{{ item.subscribers.totalCount }}</span>
          </td>
          <td>
            <v-chip color="success" outlined v-if="item.isPublic">
              Public
            </v-chip>
            <v-chip color="error" outlined v-else>
              Private
            </v-chip>
          </td>
          <td>
            <span>{{ item.product }}</span>
          </td>
          <td>
            <span>{{ item.price / 100 }} €</span>
          </td>
          <td>
            <span>{{ item.storage | filesize }}</span>
          </td>
          <td>
            <v-icon small @click="editPlan(item)">mdi-pencil</v-icon>
          </td>
        </tr>
      </template>
    </v-data-table>


    <v-dialog v-model="showEditPlanDialog" width="500">
      <v-card v-if="planToEdit">
        <v-card-title class="headline" primary-title>
          {{ planToEdit.name }}
          <v-spacer />
          <v-btn icon @click="deletePlan(planToEdit)">
            <v-icon>mdi-delete</v-icon>
          </v-btn>
        </v-card-title>

        <v-card-text>
          <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
            {{ error }}
          </v-alert>
          <v-row>
            <v-col cols="12">
              <v-text-field label="Name" outlined v-model="planToEdit.name"></v-text-field>
            </v-col>

            <v-col cols="12">
              <v-textarea
                label="Decription"
                v-model="planToEdit.description"
              ></v-textarea>
            </v-col>

            <v-col cols="12">
              <v-select
                :items="billingProducts"
                label="Product"
                outlined
                v-model="planToEdit.product"
              ></v-select>
            </v-col>

            <v-col cols="12">
              <v-text-field label="Price" outlined v-model="planToEdit.price" disabled
                prepend-icon="mdi-currency-eur" />
            </v-col>

            <v-col cols="12">
              <v-text-field label="Storage" outlined v-model="planToEdit.storage"></v-text-field>
            </v-col>

            <v-col cols="12">
              <v-text-field label="Stripe ID" outlined v-model="planToEdit.stripeId"></v-text-field>
            </v-col>

            <v-col cols="12">
               <v-switch
                v-model="planToEdit.isPublic"
                :label="`Is Public: ${planToEdit.isPublic}`"
              ></v-switch>
            </v-col>

          </v-row>
        </v-card-text>

        <v-divider></v-divider>

        <v-card-actions>
          <v-spacer />
          <v-btn text @click="closeEditPlanDialog">
            Close
          </v-btn>
          <v-btn color="primary" @click="createPlan(planToEdit)" v-if="isNewPlan">
            Create
          </v-btn>
           <v-btn color="primary" @click="updatePlan(planToEdit)" v-else>
            Update
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import * as models from '@/api/models';
import core from '@/core';
import { Method as BillingMethod } from '@/core/billing';

@Component
export default class PlansTable extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) loading!: boolean;
  @Prop({ type: Array }) plans!: models.BillingPlan[];

  // data
  error = '';
  showEditPlanDialog = false;
  planToEdit: models.BillingPlan | null = null;
  loadingInternal = false;
  isNewPlan = false;
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Name',
      value: 'name',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Subscribers',
      value: 'subscribers',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Is Public',
      value: 'isPublic',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Product',
      value: 'product',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Price',
      value: 'price',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Storage',
      value: 'storage',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Actions',
      value: 'actions',
    },
  ];
  billingProducts = ['FREE', 'BASIC', 'PRO', 'ULTRA'];

  // computed
  get isLoading(): boolean {
    return this.loading || this.loadingInternal;
  }

  // lifecycle
  // watch
  // methods
  editPlan(plan: models.BillingPlan) {
    this.planToEdit = { ...plan };
    this.showEditPlanDialog = true;
  }

  newPlan() {
    this.isNewPlan = true;
    this.planToEdit = {
      id: '',
      price: 0.0,
      name: '',
      description: '',
      isPublic: false,
      product: models.BillingProduct.Free,
      storage: 0,
      stripeId: '',
    };
    this.showEditPlanDialog = true;
  }

  closeEditPlanDialog() {
    this.showEditPlanDialog = false;
    this.planToEdit = null;
    this.isNewPlan = false;
  }

  async updatePlan(plan: models.BillingPlan) {
    this.loadingInternal = true;
    this.error = '';

    try {
      if (typeof plan.storage === 'string') {
        plan.storage = parseInt(plan.storage, 10); // eslint-disable-line
      }
      if (typeof plan.price === 'string') {
        plan.price = parseFloat(plan.price); // eslint-disable-line
      }

      const updatedPlan = await core.call(BillingMethod.UpdatedPlan, plan);
      this.closeEditPlanDialog();
      this.$emit('updated', updatedPlan);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loadingInternal = false;
    }
  }

  async createPlan(plan: models.BillingPlan) {
    this.loadingInternal = true;
    this.error = '';

    try {
      if (typeof plan.storage === 'string') {
        plan.storage = parseInt(plan.storage, 10); // eslint-disable-line
      }
      if (typeof plan.price === 'string') {
        plan.price = parseFloat(plan.price); // eslint-disable-line
      }

      const newPlan = await core.call(BillingMethod.CreatePlan, plan);
      this.closeEditPlanDialog();
      this.$emit('created', newPlan);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loadingInternal = false;
    }
  }

  async deletePlan(plan: models.BillingPlan) {
    const conf = window.confirm(`Do you really want to delete ${plan.name}?`); // eslint-disable-line
    if (!conf) {
      return;
    }

    this.loadingInternal = true;
    this.error = '';
    const input: models.DeleteBillingPlanInput = {
      id: plan.id,
    };
    try {
      await core.call(BillingMethod.DeletePlan, input);
      this.closeEditPlanDialog();
      this.$emit('deleted', plan);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loadingInternal = false;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
