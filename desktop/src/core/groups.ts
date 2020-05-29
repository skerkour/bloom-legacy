import { Group } from '@/api/models';

export enum Method {
  CreateGroup = 'groups.createGroup',
  DeleteGroup = 'groups.deleteGroup',
  FindGroups = 'groups.findGroups',
  FetchMembers = 'groups.fetchMembers',
  InviteUser = 'groups.inviteUser',
  RemoveMembers = 'groups.removeMembers',
  FetchGroups = 'groups.fetchGroups',
  FetchDetails = 'groups.fetchDetails',
  CancelInvitation = 'groups.cancelInvitation',
  FetchMyInvitations = 'groups.fetchMyInvitations',
}

export type Groups = {
  groups: Group[],
}
