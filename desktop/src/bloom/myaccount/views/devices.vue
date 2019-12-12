<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" sm="10" lg="9">
        <blm-myaccount-table-devices :loading="loading" :devices="devices" :current="current" />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import DevicesTable from '../components/devices_table.vue';

@Component({
  components: {
    'blm-myaccount-table-devices': DevicesTable,
  },
})
export default class Devices extends Vue {
  // props
  // data
  loading = false;
  current = { id: '1' };
  devices = [
    {
      id: '1',
      created_at: new Date(),
      ip: '127.0.01',
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
