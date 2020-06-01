<template>
  <v-container fluid class="text-left overflow-y-auto" style="height: 100vh">
    <v-row>
      <v-col cols="12" v-if="error">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12">
        <div class="headline">
          Members
        </div>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12">
        <v-btn color="primary" @click="openInviteDialog">
          <v-icon left>mdi-plus</v-icon> Invite
        </v-btn>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" sm="10" lg="8">
        <v-data-table
          :headers="membersHeaders"
          :items="members"
          item-key="node.username"
          :loading="loading"
          loading-text="Loading... Please wait"
          hide-default-footer>
          <template v-slot:item="{ item }">
            <tr>
              <td>
                <v-avatar color="white" v-if="item.node.avatarUrl" size="42">
                  <v-img :src="item.node.avatarUrl"></v-img>
                </v-avatar>
                <v-avatar color="white" v-else size="42">
                  <v-icon medium color="grey">mdi-account</v-icon>
                </v-avatar>
                &nbsp;
                <span>{{ item.node.displayName }} @{{ item.node.username }}</span>
              </td>
              <td>
                <span>{{ item.role }}</span>
              </td>
              <td>
                <v-menu bottom left>
                  <template v-slot:activator="{ on }">
                    <v-btn icon v-on="on" v-if="!item.isDefault">
                      <v-icon>mdi-dots-vertical</v-icon>
                    </v-btn>
                  </template>

                  <v-list>
                    <v-list-item @click="removeGroupMember(item)">
                      <v-list-item-icon>
                        <v-icon>mdi-delete</v-icon>
                      </v-list-item-icon>
                      <v-list-item-title>Remove</v-list-item-title>
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
        <div class="headline">
          Pending invitations
        </div>
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
          hide-default-footer>
          <template v-slot:item="{ item }">
            <tr>
              <td>
                <span>{{ item.invitee.displayName }} @{{ item.invitee.username }}</span>
              </td>
              <td>
                <span>{{ item.inviter.displayName }} @{{ item.inviter.username }}</span>
              </td>
              <td>
                <v-menu bottom left>
                  <template v-slot:activator="{ on }">
                    <v-btn icon v-on="on" v-if="!item.isDefault">
                      <v-icon>mdi-dots-vertical</v-icon>
                    </v-btn>
                  </template>

                  <v-list>
                    <v-list-item @click="cancelInvitation(item)">
                      <v-list-item-icon>
                        <v-icon color="error">mdi-cancel</v-icon>
                      </v-list-item-icon>
                      <v-list-item-title>Cancel invitation</v-list-item-title>
                    </v-list-item>
                  </v-list>
                </v-menu>
              </td>
            </tr>
          </template>
        </v-data-table>
      </v-col>
    </v-row>

    <blm-groups-invite-dialog :visible="showInviteDialog" :group-id="group.id" v-if="group"
      @closed="inviteDialogClosed" @invited="usersInvited"/>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmGroupsInviteDialog from '../components/invite_dialog.vue';
import {
  Group,
  GroupMemberEdge,
  Maybe,
  GroupInvitationConnection,
  GroupInvitation,
} from '@/api/models';
import core from '@/core';
import { Method } from '@/core/groups';
import { GroupsFetchMembersParams, GroupsCancelInvitationParams, GroupsRemoveMembersParams } from '@/core/messages';


@Component({
  components: {
    BlmGroupsInviteDialog,
  },
})
export default class GroupsMembersView extends Vue {
  // props
  // data
  loading = false;
  error = '';
  showInviteDialog = false;
  group: Group | null = null;
  membersHeaders = [
    {
      align: 'left',
      sortable: true,
      text: 'User',
      value: 'node.displayName',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Role',
      value: 'role',
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
      text: 'Invitee',
      value: 'node.invitee.displayName',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Inviter',
      value: 'node.inviter.displayName',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Actions',
      value: 'actions',
    },
  ];

  // computed
  get members(): GroupMemberEdge[] {
    if (this.group) {
      return this.group.members!.edges!.map((edge: Maybe<GroupMemberEdge>) => edge!);
    }
    return [];
  }

  get invitations(): GroupInvitation[] {
    if (this.group) {
      return this.group.invitations!.nodes;
    }
    return [];
  }

  // lifecycle
  created() {
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.loading = true;
    const params: GroupsFetchMembersParams = {
      groupID: this.$route.params.groupId,
    };

    try {
      this.group = await core.call(Method.FetchMembers, params);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  openInviteDialog() {
    this.showInviteDialog = true;
  }

  inviteDialogClosed() {
    this.showInviteDialog = false;
  }

  usersInvited(invitations: GroupInvitationConnection) {
    if (this.group) {
      this.group.invitations = invitations;
    }
  }

  async removeGroupMember(member: GroupMemberEdge) {
    this.loading = true;
    this.error = '';
    const params: GroupsRemoveMembersParams = {
      groupID: this.group!.id!,
      username: member!.node!.username,
    };

    try {
      const res: Group = await core.call(Method.RemoveMembers, params);
      this.group!.members = res.members;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async cancelInvitation(invitation: GroupInvitation) {
    this.loading = true;
    this.error = '';
    console.log(invitation);
    const params: GroupsCancelInvitationParams = {
      invitationID: invitation.id,
    };

    try {
      await core.call(Method.CancelInvitation, params);
      this.group!.invitations!.nodes = this.group!.invitations!.nodes
        .filter((invit: GroupInvitation) => invitation.id !== invit.id);
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
