import { Group } from '@/api/models';

export enum Method {
  CreateGroup = 'groups.createGroup',
  DeleteGroup = 'groups.deleteGroup',
  FindGroups = 'groups.findGroups',
  FetchGroupMembers = 'groups.fetchGroupMembers',
  InviteUsers = 'groups.inviteUsers',
  RemoveMembers = 'groups.removeMembers',
  FetchGroups = 'groups.fetchGroups',
}

export type Groups = {
  groups: Group[],
}

export type FetchGroupMembersParams = {
  id: string,
}
