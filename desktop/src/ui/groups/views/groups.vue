<template>
  <v-container fluid class="text-left">
    <v-row>
      <v-alert type="error" :value="error !== ''">
        {{ error }}
      </v-alert>
    </v-row>

    <v-row>
      <v-toolbar flat dense>
        <v-btn color="primary" @click="openNewGroupDialog">
          <v-icon left>mdi-plus</v-icon>New
        </v-btn>
      </v-toolbar>
    </v-row>

    <v-row>
      <v-col cols="12" sm="10" lg="8">
        <v-data-table
          :headers="groupsHeaders"
          :items="groups"
          item-key="id"
          :loading="loading"
          loading-text="Loading... Please wait"
          hide-default-footer>
          <template v-slot:item="{ item }" >
            <tr @click="goto(`/groups/${item.id}/members`)" class="blm-pointer">
              <td>
               <v-avatar color="white" v-if="item.avatarUrl" size="42">
                  <v-img :src="item.avatarUrl"></v-img>
                </v-avatar>
                <v-avatar color="white" v-else size="42">
                  <v-icon medium color="grey">mdi-account-group</v-icon>
                </v-avatar>
                &nbsp;
                <span>{{ item.name }}</span>
              </td>
              <td>
                <span>{{ item.description }}</span>
              </td>
            </tr>
          </template>
        </v-data-table>
      </v-col>
    </v-row>

  <blm-groups-new-group-dialog :visible="showNewGroupDialog"
      @closed="newGroupDialogClosed" @created="groupCreated" />

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Group } from '@/api/models';
import core from '@/core';
import { Method, Groups } from '@/core/groups';
import BlmGroupsNewGroupDialog from '../components/new_group_dialog.vue';

@Component({
  components: {
    BlmGroupsNewGroupDialog,
  },
})
export default class GroupsView extends Vue {
  // props
  // data
  error = '';
  groups: Group[] = [];
  loading = false;
  showNewGroupDialog = false;
  groupsHeaders = [
    {
      align: 'left',
      sortable: true,
      text: 'Group',
      value: 'name',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Description',
      value: 'description',
    },
  ]

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
      const res: Groups = await core.call(Method.FindGroups, core.Empty);
      this.groups = res.groups;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  openNewGroupDialog() {
    this.showNewGroupDialog = true;
  }

  groupCreated(group: Group) {
    this.groups.push(group);
  }

  newGroupDialogClosed() {
    this.showNewGroupDialog = false;
  }

  goto(path: string) {
    this.$router.push({ path });
  }
}
</script>


<style lang="scss" scoped>
</style>
