<template>
  <v-container>
    <v-layout row wrap justify-center>
      <v-flex xs12 md10 v-if="logs">
        <v-data-table
          :headers="headers"
          :items="logs"
          class="elevation-0"
          :total-items="total_logs"
          :loading="is_loading"
          disable-initial-sort
          :rows-per-page-items="[ items_per_page ]"
          :pagination.sync="pagination">
          <template slot="items" slot-scope="props">
            <td class="text-xs-center">{{ props.item.timestamp | date }}</td>
            <td class="text-xs-center">{{ props.item.type }}</td>
          </template>
        </v-data-table>
        <div class="text-xs-center pt-2">
          <v-pagination v-model="pagination.page" :length="pages"></v-pagination>
        </div>
      </v-flex>
      <v-flex xs12 sm10 md8 xl7 v-else>
        <v-progress-circular
        :size="50"
        color="primary"
        indeterminate
        ></v-progress-circular>
      </v-flex>
    </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class Audit extends Vue {
  error = '';
  is_loading = false;
  items_per_page = 15;
  headers = [
    {
      align: 'center',
      sortable: false,
      text: 'Time',
      value: 'timestamp',
    },
    {
      align: 'center',
      sortable: false,
      text: 'Action',
      value: 'action',
    },
  ];
  pagination: any = {};
  logs: any[] = [];
  total_logs = 0;

  @Watch('pagination', { deep: true })
  on_pagination_change() {
    this.fetch_data();
  }

  created() {
    this.fetch_data();
  }

  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    try {
      const data = await api.query(
        api.ACCOUNT_GRAPHQL,
        `query ($filter: FilterInput) {
          me {
            logs(filter: $filter) {
              total_count
              hits {
                id
                type
                action
                timestamp
                metadata
              }
            }
          }
        }
        `,
        {
          filter: {
            limit: this.items_per_page,
            offset: this.pagination.page * this.items_per_page - this.items_per_page,
          },
        },
      );
      this.total_logs = data.me.logs.total_count;
      this.logs = data.me.logs.hits;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>
