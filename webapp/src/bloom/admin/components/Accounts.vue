<template>
    <v-layout row wrap>
      <v-flex xs12>
        <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
          {{ error }}
        </v-alert>
      </v-flex>

      <v-flex xs12>
        <v-container>
          <v-layout row wrap>
            <v-flex xs12 sm5>
              <v-text-field
                label="Username"
                v-model="username"
              ></v-text-field>
            </v-flex>

            <v-flex xs12 sm5>
              <v-text-field
                label="Email"
                v-model="email"
              ></v-text-field>
            </v-flex>

            <v-flex xs12 sm2>
              <v-btn @click="fetch_data" color="primary">
              <v-icon left dark>mdi-magnify</v-icon>
              Search
              </v-btn>
            </v-flex>
          </v-layout>
        </v-container>
      </v-flex>

      <v-flex xs12>
        <v-data-table
          :headers="headers"
          :items="accounts"
          hide-actions
          :loading="is_loading"
          :pagination.sync="pagination"
          :total-items="total">
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
              <v-chip color="success" outline v-if="!props.item.is_disabled">Active</v-chip>
              <v-chip v-else outline>Disabled</v-chip>

              <v-chip color="error" dark v-if="props.item.deleted_at">Deleted</v-chip>
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
          <v-pagination v-model="pagination.page" :length="pages" :total-visible="8" @input="paginate"></v-pagination>
        </div>
      </v-flex>

    </v-layout>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import router from '@/bloom/kernel/router';


const LIMIT = 25;

@Component
export default class Accounts extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  username = '';
  email = '';
  offset = 0;
  total = 0;
  accounts: any[] = [];
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
  get pages() {
    if (!this.total) {
      return 0;
    }

    return Math.ceil(this.total / LIMIT);
  }


  // lifecycle
  created() {
    this.pagination.rowsPerPage = -1;
    const page = this.$route.query.page;
    if (page) {
      const parsed_page = Number(page);
      if (!isNaN(parsed_page)) {
        this.pagination.page = parsed_page;
      }
    }
    this.fetch_data();
  }


  // watch
  // @Watch('pagination', { deep: true })
  async paginate() {
    router.push({ query: { page: this.pagination.page }});
    return this.fetch_data();
  }


  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    const offset = this.pagination.page ? (this.pagination.page - 1) * LIMIT : 0;
    const username = this.username.trim() === '' ? undefined : this.username.trim();
    const email = this.email.trim() === '' ? undefined : this.email.trim();
    try {
      const res = await api.get(`${api.ADMIN}/v1/accounts`, {
        params: {
          email,
          limit: LIMIT,
          offset,
          username,
        },
      });
      this.accounts = res.hits;
      this.total = res.total;
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
