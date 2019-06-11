<template>
  <div>
    <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
      {{ error }}
    </v-alert>
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class Accounts extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  accounts: any[] = [];
  selected: any[] = [];
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Name',
      value: 'name',
      width: '50%',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Progress',
      value: 'progress',
    },
    { text: 'Actions', sortable: false },
  ];


  // computed
  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data(loading?: boolean) {
    if (loading !== false) {
      this.error = '';
      this.is_loading = true;
    }
    try {
      this.accounts = await api.get(`${api.MYACCOUNT}/v1/accounts`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
</style>
