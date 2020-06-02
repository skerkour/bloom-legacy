<template>
  <v-container fluid class="text-left overflow-y-auto" style="height: 100vh">
    <v-row>
      <v-col cols="12">
        <h2 class="headline">
          My Groups
        </h2>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" sm="10" lg="8">
        <v-alert type="error" :value="groupsError !== ''">
          {{ groupsError }}
        </v-alert>
      </v-col>
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
          :items="$store.state.groups"
          item-key="id"
          :loading="groupsLoading"
          loading-text="Loading... Please wait"
          hide-default-footer>
          <template v-slot:item="{ item }" >
            <tr>
              <td @click="goto(`/groups/${item.id}/members`)" class="blm-pointer">
               <v-avatar color="white" v-if="item.avatarUrl" size="42">
                  <v-img :src="item.avatarUrl"></v-img>
                </v-avatar>
                <v-avatar color="white" v-else size="42">
                  <v-icon medium color="grey">mdi-account-group</v-icon>
                </v-avatar>
                &nbsp;
                <span>{{ item.name }}</span>
              </td>
              <td @click="goto(`/groups/${item.id}/members`)" class="blm-pointer">
                <span>{{ item.description }}</span>
              </td>
              <td>
                <v-menu bottom left>
                  <template v-slot:activator="{ on }">
                    <v-btn icon v-on="on">
                      <v-icon>mdi-dots-vertical</v-icon>
                    </v-btn>
                  </template>

                  <v-list>
                    <v-list-item @click="quitGroup(item)">
                      <v-list-item-icon>
                        <v-icon>mdi-cancel</v-icon>
                      </v-list-item-icon>
                      <v-list-item-title>Quit group</v-list-item-title>
                    </v-list-item>

                    <v-list-item @click="deleteGroup(item)">
                      <v-list-item-icon>
                        <v-icon color="red">mdi-delete</v-icon>
                      </v-list-item-icon>
                      <v-list-item-title>Delete group</v-list-item-title>
                    </v-list-item>
                  </v-list>
                </v-menu>
              </td>
            </tr>
          </template>
        </v-data-table>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12">
        <h2 class="headline">
          My Invitations
        </h2>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" sm="10" lg="8">
        <v-alert type="error" :value="invitationsError !== ''">
          {{ invitationsError }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row>
      <v-col cols="12" sm="10" lg="8">
        <v-data-table
          :headers="invitationsHeaders"
          :items="invitations"
          item-key="invitee.username"
          :loading="loading"
          loading-text="Loading... Please wait"
          no-data-text="No invitation"
          hide-default-footer>
          <template v-slot:item="{ item }">
            <tr>
              <td>
                <span>{{ item.group.name }}</span>
              </td>
              <td>
                <span>{{ item.inviter.displayName }} @{{ item.inviter.username }}</span>
              </td>
              <td>
                <v-menu bottom left>
                  <template v-slot:activator="{ on }">
                    <v-btn icon v-on="on">
                      <v-icon>mdi-dots-vertical</v-icon>
                    </v-btn>
                  </template>

                  <v-list>
                    <v-list-item @click="acceptInvitation(item)">
                      <v-list-item-icon>
                        <v-icon color="success">mdi-check</v-icon>
                      </v-list-item-icon>
                      <v-list-item-title>Join group</v-list-item-title>
                    </v-list-item>

                    <v-list-item @click="declineInvitation(item)">
                      <v-list-item-icon>
                        <v-icon color="red">mdi-cancel</v-icon>
                      </v-list-item-icon>
                      <v-list-item-title>Decline invitation</v-list-item-title>
                    </v-list-item>
                  </v-list>
                </v-menu>
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
import { Group, GroupInvitation, User } from '@/api/models';
import core from '@/core';
import { Method } from '@/core/groups';
import BlmGroupsNewGroupDialog from '../components/new_group_dialog.vue';
import { GroupsDeclineInvitationParams, GroupsQuitParams, GroupsDeleteParams } from '@/core/messages';
import { Mutations } from '@/store';

@Component({
  components: {
    BlmGroupsNewGroupDialog,
  },
})
export default class GroupsView extends Vue {
  // props
  // data
  groupsError = '';
  invitationsError = '';
  invitations: GroupInvitation[] = [];
  groupsLoading = false;
  invitationsLoading = false;
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
    {
      align: 'left',
      sortable: false,
      text: 'Actions',
      value: 'actions',
    },
  ];
  invitationsHeaders = [
    {
      align: 'left',
      sortable: true,
      text: 'Group',
      value: 'group.name',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Inviter',
      value: 'inviter.displayName',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Actions',
      value: 'actions',
    },
  ];

  // computed
  // lifecycle
  created() {
    this.fetchMyInvitations();
  }

  // watch
  // methods
  async fetchMyInvitations() {
    this.invitationsError = '';
    this.invitationsLoading = true;

    try {
      const res: User = await core.call(Method.FetchMyInvitations, core.Empty);
      this.invitations = res.groupInvitations!.nodes;
    } catch (err) {
      this.invitationsError = err.message;
    } finally {
      this.invitationsLoading = false;
    }
  }

  async acceptInvitation(invitation: GroupInvitation) {
    this.invitationsError = '';
    this.invitationsLoading = true;

    try {
      const res: Group = await core.call(Method.AcceptInvitation, invitation);
      this.invitations = this.invitations
        .filter((invit: GroupInvitation) => invitation.id !== invit.id);
      this.$store.commit(Mutations.ADD_GROUP.toString(), res);
      this.$router.push({ path: '/sync', query: { redirect: encodeURIComponent('/groups') } });
    } catch (err) {
      this.invitationsError = err.message;
    } finally {
      this.invitationsLoading = false;
    }
  }

  async declineInvitation(invitation: GroupInvitation) {
    this.invitationsError = '';
    this.invitationsLoading = true;
    const params: GroupsDeclineInvitationParams = {
      invitationID: invitation.id,
    };

    try {
      await core.call(Method.DeclineInvitation, params);
      this.invitations = this.invitations
        .filter((invit: GroupInvitation) => invitation.id !== invit.id);
    } catch (err) {
      this.invitationsError = err.message;
    } finally {
      this.invitationsLoading = false;
    }
  }

  async quitGroup(group: Group) {
    this.groupsError = '';
    this.groupsLoading = true;
    const params: GroupsQuitParams = {
      groupID: group.id!,
    };

    try {
      await core.call(Method.QuitGroup, params);
      this.$store.commit(Mutations.REMOVE_GROUP.toString(), group.id);
    } catch (err) {
      this.groupsError = err.message;
    } finally {
      this.groupsLoading = false;
    }
  }

  async deleteGroup(group: Group) {
    this.groupsError = '';
    this.groupsLoading = true;
    const params: GroupsDeleteParams = {
      groupID: group.id!,
    };

    try {
      await core.call(Method.DeleteGroup, params);
      this.$store.commit(Mutations.REMOVE_GROUP.toString(), group.id);
    } catch (err) {
      this.groupsError = err.message;
    } finally {
      this.groupsLoading = false;
    }
  }

  openNewGroupDialog() {
    this.showNewGroupDialog = true;
  }

  groupCreated(group: Group) {
    this.$store.commit(Mutations.ADD_GROUP.toString(), group);
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
