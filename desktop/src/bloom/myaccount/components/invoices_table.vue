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
        <td>
          <span>{{ item.type }}</span>
        </td>
      </template>
    </v-data-table>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class InvoicesTable extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) loading!: boolean;
  @Prop({ type: Array }) invoices!: any[];

  // data
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Type',
      value: 'type',
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
