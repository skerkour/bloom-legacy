<template>
  <v-container fluid class="text-left">
    <v-data-table
      :headers="headers"
      :items="plans"
      item-key="id"
      :loading="loading"
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
          <span>Actions</span>
        </td>
      </template>
    </v-data-table>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import * as models from '@/api/models';

@Component
export default class PlansTable extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) loading!: boolean;
  @Prop({ type: Array }) plans!: models.BillingPlan[];

  // data
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
  // lifecycle
  // watch
  // invoices
}
</script>


<style lang="scss" scoped>
</style>
