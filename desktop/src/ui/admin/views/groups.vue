<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row class="text-left">
      <v-col cols="12" class="pt-0">
        <v-data-table
          :headers="headers"
          :items="groups"
          item-key="id"
          :loading="loading"
          loading-text="Loading... Please wait"
          hide-default-footer>
          <template v-slot:no-data>
            <p class="text-center">
              No group
            </p>
          </template>

          <template v-slot:item="{ item }">
            <tr>
              <td>
                <span>{{ item.name }}</span>
              </td>
              <td>
                <v-tooltip bottom>
                  <template v-slot:activator="{ on }">
                    <v-btn icon v-on="on" :to="`/admin/groups/${item.id}`">
                      <v-icon>mdi-magnify</v-icon>
                    </v-btn>
                  </template>
                  <span>Inspect group</span>
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
import { GroupEdge, Maybe, Group } from '@/api/models';
import { Method } from '@/core/groups';
import core from '@/core';

@Component
export default class AdminGroupsView extends Vue {
  // props
  // data
  error = '';
  loading = false;
  groups: Group[] = [];
  totalGroupCount = 0;
  usernameSearch = '';
  headers = [
    {
      align: 'left',
      sortable: false,
      text: 'Name',
      value: 'name',
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
      const res = await core.call(Method.FetchGroups, core.Empty);
      this.groups = res.edges.map((edge: Maybe<GroupEdge>) => edge!.node!);
      this.totalGroupCount = res.totalCount;
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
