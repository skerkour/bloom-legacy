<template>
  <v-container fluid class="text-left">
    <v-row>
      <v-col cols="12" v-if="error">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
      <v-col cols="12" v-if="loading">
        <v-progress-circular
          indeterminate
          color="primary"
          size="50"
        />
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12">
        <v-btn color="primary" @click="openInviteDialog">
          <v-icon left>mdi-plus</v-icon> Invite
        </v-btn>
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12" sm="10" md="8" lg="6">
        <v-list three-line>
          <template v-for="member in members">
            <v-list-item :key="member.node.id">
              <v-list-item-avatar color="white" v-if="member.node.avatarUrl">
                <v-img :src="member.node.avatarUrl"></v-img>
              </v-list-item-avatar>
              <v-list-item-avatar color="white" v-else>
                <v-icon medium color="grey">mdi-account</v-icon>
              </v-list-item-avatar>

              <v-list-item-content>
                <v-list-item-title>{{ member.node.displayName }}</v-list-item-title>
                <v-list-item-subtitle>@{{ member.node.username }}</v-list-item-subtitle>
              </v-list-item-content>

              <v-list-item-action>
                <v-list-item-action-text>
                  {{ member.role }}
                </v-list-item-action-text>
              </v-list-item-action>
            </v-list-item>
          </template>
        </v-list>
      </v-col>
    </v-row>

    <blm-groups-invite-dialog :visible="showInviteDialog"
      @closed="inviteDialogClosed" />
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmGroupsInviteDialog from '../components/invite_dialog.vue';
import { Group, GroupMemberEdge, Maybe } from '@/api/models';
import core from '@/core';
import { Method, FetchGroupMembersParams } from '@/core/groups';

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

  // computed
  get members(): GroupMemberEdge[] {
    if (this.group) {
      return this.group.members!.edges!.map((edge: Maybe<GroupMemberEdge>) => edge!);
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
    const params: FetchGroupMembersParams = {
      id: this.$route.params.groupId,
    };

    try {
      this.group = await core.call(Method.FetchGroupMembers, params);
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
}
</script>


<style lang="scss" scoped>
</style>
