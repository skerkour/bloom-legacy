<template>
  <v-container fluid>
    <v-data-table
      :headers="headers"
      :items="methods"
      item-key="id"
      :loading="loading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No payment method.
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
export default class Security extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) loading!: boolean;
  @Prop({ type: Array, default: [] }) methods!: any[];

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
  // methods
}
</script>


<style lang="scss" scoped>
</style>
