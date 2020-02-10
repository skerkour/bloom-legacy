<template>
  <v-container fluid class="text-left">
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>
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
        <td>
          <span>{{ item.name }}</span>
        </td>
        <td>
          <v-chip color="success" outlined v-if="item.isActive">
            Active
          </v-chip>
          <v-chip color="error" outlined v-else>
            Inactive
          </v-chip>
        </td>
        <td>
          <span>{{ item.tier }}</span>
        </td>
        <td>
          <span>{{ item.storage }}</span>
        </td>
        <td>
          <span>{{ item.description | truncate }}</span>
        </td>
        <td>
          <v-icon small @click="editPlan(item)">mdi-pencil</v-icon>
        </td>
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
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
          incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud
          exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure
          dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
          Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit
          anim id est laborum.
        </v-card-text>

        <v-divider></v-divider>

        <v-card-actions>
          <v-spacer />
          <v-btn text @click="closeEditPlanDialog">
            Close
          </v-btn>
           <v-btn color="success" @click="planToEdit">
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
import AdminMethods from '@/bloom/admin/core/methods';

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
      text: 'Is active',
      value: 'isActive',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Tier',
      value: 'tier',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Storage',
      value: 'storage',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Description',
      value: 'description',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Actions',
      value: 'actions',
    },
  ];

  // computed
  get isLoading(): boolean {
    return this.loading || this.loadingInternal;
  }
  // lifecycle
  // watch
  // methods
  editPlan(plan: models.BillingPlan) {
    this.showEditPlanDialog = true;
    this.planToEdit = plan;
  }

  closeEditPlanDialog() {
    this.planToEdit = null;
    this.showEditPlanDialog = false;
  }

  async updatePlan(plan: models.BillingPlan) {
    console.log(plan);
    this.closeEditPlanDialog();
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
      this.plans = await core.call(AdminMethods.DeleteBillingPlan, input);
      this.closeEditPlanDialog();
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
