<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12" sm="9" md="8" class="pb-0">
        <v-text-field
          v-model="usernameSearch"
          placeholder="Search by username"
          outlined
          append-icon="mdi-magnify"
          @click:append="searchUser"
          @keyup.enter.native="searchUser"
        />
      </v-col>
    </v-row>

    <v-row class="text-left">
      <v-col cols="12" class="pt-0">
        <v-data-table
          :headers="headers"
          :items="users"
          item-key="id"
          :loading="loading"
          loading-text="Loading... Please wait"
          hide-default-footer>
          <template v-slot:no-data>
            <p class="text-center">
              No user
            </p>
          </template>

          <template v-slot:item="{ item }">
            <tr>
              <td>
                <span>{{ item.username }}</span>
              </td>
              <td>
                <v-chip color="error" outlined v-if="item.disabled_at">Disabled</v-chip>
                <v-chip color="success" outlined v-else>Active</v-chip>
              </td>
              <td>
                <v-tooltip bottom>
                  <template v-slot:activator="{ on }">
                    <v-btn icon v-on="on" :to="`/admin/users/${item.username}`">
                      <v-icon>mdi-magnify</v-icon>
                    </v-btn>
                  </template>
                  <span>Inspect user</span>
                </v-tooltip>
              </td>
            </tr>
          </template>
        </v-data-table>
      </v-col>
    </v-row>


  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { User, UserEdge, Maybe } from '@/api/models';
import { Method, FetchUserParams } from '@/core/users';
import core from '@/core';

@Component
export default class AdminUsersView extends Vue {
  // props
  // data
  error = '';
  loading = false;
  users: User[] = [];
  totalUserCount = 0;
  usernameSearch = '';
  headers = [
    {
      align: 'left',
      sortable: false,
      text: 'Username',
      value: 'username',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Active',
      value: 'disabled_at',
    },
    { text: 'Actions', sortable: false },
  ];

  // computed
  // lifecycle
  created() {
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.loading = true;

    try {
      const res = await core.call(Method.FetchUsers, core.Empty);
      this.users = res.edges.map((edge: Maybe<UserEdge>) => edge!.node!);
      this.totalUserCount = res.totalCount;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async searchUser() {
    this.error = '';
    this.loading = true;
    const params: FetchUserParams = {
      username: this.usernameSearch,
    };

    try {
      const res = await core.call(Method.FetchUser, params);
      if (!res) {
        this.users = [];
      }
      this.users = [res];
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
