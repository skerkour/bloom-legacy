<template>
  <v-container fluid>
    <v-data-table
      :headers="headers"
      :items="invoices"
      item-key="id"
      :loading="loading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No invoice.
        </p>
      </template>

      <template v-slot:item="{ item }" class="text-left">
        <tr>
          <td>
            <span>{{ item.createdAt | date }}</span>
          </td>
          <td>
            <span>{{ item.amount / 100 }}€</span>
          </td>
          <td>
            <v-chip color="success" outlined v-if="item.paid">Paid</v-chip>
            <v-chip color="error" outlined v-else>Unpaid</v-chip>
          </td>
          <td>
            <v-btn text icon @click="openStripePdfUrl(item.stripePdfUrl)">
              <v-icon>mdi-file-pdf-outline</v-icon>
            </v-btn>
          </td>
        </tr>
      </template>
    </v-data-table>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { shell } from 'electron';
import * as models from '@/api/models';

@Component
export default class InvoicesTable extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) loading!: boolean;
  @Prop({ type: Array }) invoices!: models.Invoice[];

  // data
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Date',
      value: 'createdAt',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Amount',
      value: 'amount',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Paid',
      value: 'paid',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Download',
      value: 'download',
    },
  ];

  // computed
  // lifecycle
  // watch
  // invoices
  openStripePdfUrl(url: string) {
    shell.openExternal(url);
  }
}
</script>


<style lang="scss" scoped>
</style>
