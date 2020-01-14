<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" sm="9" md="8" class="pb-0">
        <v-text-field
          placeholder="Search by email or username"
          outlined
          append-icon="mdi-magnify"
          @click:append="search"
          @keyup.enter.native="search"
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
                  <v-btn icon v-on="on" :to="`/admin/users/${item.id}`">
                    <v-icon>mdi-magnify</v-icon>
                  </v-btn>
                </template>
                <span>Inspect user</span>
              </v-tooltip>
            </td>
          </template>
        </v-data-table>
      </v-col>
    </v-row>


  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Accounts extends Vue {
  // props
  // data
  error = '';
  loading = false;
  users = [
    {
      id: '1',
      username: 'sylvain',
      disabled_at: null,
    },
  ];
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
  // watch
  // methods
  search() {
  }
}
</script>


<style lang="scss" scoped>
</style>
