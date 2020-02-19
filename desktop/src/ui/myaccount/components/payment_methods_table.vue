<template>
  <v-container fluid>
    <v-data-table
      :headers="headers"
      :items="paymentMethods"
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
        <tr>
          <td>
            <span><v-icon>mdi-credit-card-outline</v-icon></span>
          </td>
          <td>
            xxxx-{{ item.cardLast4 }}
          </td>
          <td>
            {{ item.cardExpirationMonth }} / {{ item.cardExpirationYear }}
          </td>
          <td>
            <v-chip color="success" outlined v-if="item.isDefault">
              Default
            </v-chip>
          </td>
          <td>
            <v-menu bottom left>
              <template v-slot:activator="{ on }">
                <v-btn icon v-on="on" v-if="!item.isDefault">
                  <v-icon>mdi-dots-vertical</v-icon>
                </v-btn>
              </template>

              <v-list>
                <v-list-item @click="onRemove(item)">
                  <v-list-item-icon>
                    <v-icon>mdi-delete</v-icon>
                  </v-list-item-icon>
                  <v-list-item-title>Remove</v-list-item-title>
                </v-list-item>
                <v-list-item>
                  <v-list-item-icon>
                    <v-icon>mdi-star</v-icon>
                  </v-list-item-icon>
                  <v-list-item-title>Set as default</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>
          </td>
        </tr>
      </template>
    </v-data-table>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import * as models from '@/api/models';

@Component
export default class Security extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) loading!: boolean;
  @Prop({ type: Array }) paymentMethods!: models.PaymentMethod[];

  // data
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Type',
      value: 'type',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Number',
      value: 'cardLast4',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Expiration date',
      value: 'cardExpirationYear',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Is Default',
      value: 'isDefualt',
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
  // methods
  onRemove(paymentMethod: any) {
    this.$emit('removed', paymentMethod);
  }
}
</script>


<style lang="scss" scoped>
</style>
