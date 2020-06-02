/* eslint-disable */
export enum Method {
  CreateGroup = 'groups.createGroup',
  DeleteGroup = 'groups.deleteGroup',
  FindGroups = 'groups.findGroups',
  FetchGroups = 'groups.fetchGroups',
  FetchDetails = 'groups.fetchDetails',
  QuitGroup = 'groups.quitGroup',
  // members
  FetchMembers = 'groups.fetchMembers',
  InviteUser = 'groups.inviteUser',
  RemoveMembers = 'groups.removeMembers',
  // invitations
  CancelInvitation = 'groups.cancelInvitation',
  FetchMyInvitations = 'groups.fetchMyInvitations',
  AcceptInvitation = 'groups.acceptInvitation',
  DeclineInvitation = 'groups.declineInvitation',
}
