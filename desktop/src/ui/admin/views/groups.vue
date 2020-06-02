<template>
  <v-container fluid style="height: 100vh" class="overflow-y-auto">
    <v-row justify="center">
      <v-col cols="12">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row class="text-left">
      <v-col cols="12" class="pt-0">
        <blm-groups-simple-table :loading="loading" :groups="groups" inspect-url="/admin/groups" />
      </v-col>
    </v-row>


  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Group } from '@/api/models';
import { Method } from '@/core/groups';
import core from '@/core';
import BlmGroupsSimpleTable from '@/ui/groups/components/simple_table.vue';


@Component({
  components: {
    BlmGroupsSimpleTable,
  },
})
export default class AdminGroupsView extends Vue {
  // props
  // data
  error = '';
  loading = false;
  groups: Group[] = [];
  totalGroupCount = 0;
  usernameSearch = '';

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
      this.groups = res.nodes;
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
