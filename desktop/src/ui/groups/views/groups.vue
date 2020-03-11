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

    <v-row justify="center">
      <v-col cols="12" sm="10" md="8" lg="6">
        <v-list three-line>
          <template v-for="group in groups">
            <v-list-item :key="group.id" :to="`/groups/${group.id}/members`">
              <v-list-item-avatar color="white">
                <v-icon medium color="grey">mdi-account-group</v-icon>
              </v-list-item-avatar>

              <v-list-item-content>
                <v-list-item-title>{{ group.name }}</v-list-item-title>
                <v-list-item-subtitle>{{ group.description }}</v-list-item-subtitle>
              </v-list-item-content>
            </v-list-item>
          </template>
        </v-list>
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
}
</script>


<style lang="scss" scoped>
</style>
