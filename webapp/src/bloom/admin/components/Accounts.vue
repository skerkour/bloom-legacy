<template>
  <div>
    <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
      {{ error }}
    </v-alert>

    <v-data-table
      :headers="headers"
      :items="accounts"
      item-key="id"
      hide-actions
      :loading="is_loading"
      :pagination.sync="pagination"
      v-model="selected">
      <template slot="no-data">
        <p class="text-xs-center">
          No accounts.
        </p>
      </template>
      <template v-slot:items="props">
        <td class="text-xs-left">
          <span>{{ props.item.username }}</span>
        </td>
        <td class="text-xs-left">
          <v-btn outline small color="success" :ripple="false" v-if="!props.item.is_disabled">
            active
          </v-btn>
          <v-btn v-else outline small :ripple="false" color="error">
            disabled
          </v-btn>
        </td>
        <td class="justify-left">
          <v-tooltip bottom>
            <v-btn flat icon small color="grey darken-1" slot="activator"
              :to="`/admin/accounts/${props.item.id}`">
              <v-icon small>mdi-magnify</v-icon>
            </v-btn>
            <span>View account</span>
          </v-tooltip>
        </td>
      </template>
    </v-data-table>
    <div class="text-xs-center pt-2">
      <v-pagination v-model="pagination.page" :length="pages" :total-visible="8"
       @input="fetch_data"></v-pagination>
    </div>
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';

const LIMIT = 1;

@Component
export default class Accounts extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  offset = 0;
  pages = 0;
  accounts: any[] = [];
  selected: any[] = [];
  pagination: any = {};
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Username',
      value: 'username',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Active',
      value: 'is_disabled',
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
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    const offset = this.pagination.page ? (this.pagination.page - 1) * LIMIT : 0;
    try {
      const res = await api.get(`${api.ADMIN}/v1/accounts`, {
        params: {
          limit: LIMIT,
          offset,
        },
      });
      this.accounts = res.hits;
      this.set_total(res.total);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  set_total(value: number) {
    this.pagination.totalItems = value;
    this.pages = Math.ceil(value / LIMIT);
  }
}
</script>


<style scoped lang="scss">
</style>
