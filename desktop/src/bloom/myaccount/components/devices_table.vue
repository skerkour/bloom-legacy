<template>
  <v-container fluid class="text-left">
    <v-data-table
      :headers="headers"
      :items="devices"
      item-key="id"
      :loading="loading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No device
        </p>
      </template>

      <template v-slot:item="{ item }">
        <td>
          <span>{{ item.created_at | date }}</span>
        </td>
        <td>
          <span>{{ item.ip }}</span>
        </td>
        <td>
          <v-btn
            outlined
            small
            color="success"
            :ripple="false"
            v-if="item.id === current.id">
              Current
          </v-btn>
          <v-btn
            v-else
            color="error"
            @click="revokeSession(item.id)"
            :loading="item.isLoading">
              Revoke access
          </v-btn>
        </td>
      </template>
    </v-data-table>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class DevicesTable extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) loading!: boolean;
  @Prop({ type: Array }) devices!: any[];
  @Prop({ type: Object }) current!: any;


  // data
  headers = [
    {
      sortable: true,
      text: 'Signed in',
      value: 'created_at',
    },
    {
      sortable: false,
      text: 'IP',
      value: 'ip',
    },
    {
      sortable: false,
      text: 'Actions',
      value: 'id',
    },
  ];
  // computed
  // lifecycle
  // watch
  // methods
  revokeSession() {
  }
}
</script>


<style lang="scss" scoped>
</style>
