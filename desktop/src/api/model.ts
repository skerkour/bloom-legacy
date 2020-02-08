export type Maybe<T> = T | null;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string,
  String: string,
  Boolean: boolean,
  Int: number,
  Float: number,
  Time: any,
  Int64: any,
  Bytes: any,
};

export type AcceptGroupInvitationInput = {
  id: Scalars['String'],
};

export type AddPaymentMethodInput = {
  stripeId: Scalars['String'],
  groupId?: Maybe<Scalars['String']>,
};

export type BillingPlan = {
   __typename?: 'BillingPlan',
  id: Scalars['String'],
  price: Scalars['Float'],
  name: Scalars['String'],
  description: Scalars['String'],
  isActive: Scalars['Boolean'],
  tier: BillingPlanTier,
  allowedStorage: Scalars['Int64'],
};

export type BillingPlanInput = {
  id?: Maybe<Scalars['String']>,
  name: Scalars['String'],
  price: Scalars['Float'],
  tier: BillingPlanTier,
  stripeId: Scalars['String'],
  description: Scalars['String'],
  isActive?: Maybe<Scalars['Boolean']>,
};

export enum BillingPlanTier {
  Free = 'FREE',
  Basic = 'BASIC',
  Pro = 'PRO',
  Ultra = 'ULTRA'
}


export type CancelGroupInvitationInput = {
  id: Scalars['String'],
};

export type ChangeBillingPlanInput = {
  id: Scalars['String'],
  userId?: Maybe<Scalars['String']>,
  groupId?: Maybe<Scalars['String']>,
};

export type ChangeDefaultPaymentMethodInput = {
  id: Scalars['String'],
};

export type CompleteRegistrationInput = {
  id: Scalars['String'],
  username: Scalars['String'],
  authKey: Scalars['Bytes'],
  device: SessionDeviceInput,
};

export type CreateGroupInput = {
  name: Scalars['String'],
  description: Scalars['String'],
  usersToInvite: Array<Scalars['String']>,
};

export type DeclineGroupInvitationInput = {
  id: Scalars['String'],
};

export type DeleteBillingPlanInput = {
  id: Scalars['String'],
};

export type DeleteGroupInput = {
  id: Scalars['String'],
};

export type Group = {
   __typename?: 'Group',
  id?: Maybe<Scalars['String']>,
  createdAt?: Maybe<Scalars['Time']>,
  name: Scalars['String'],
  description: Scalars['String'],
  members?: Maybe<Array<GroupMember>>,
  invitations?: Maybe<Array<GroupInvitation>>,
};

export type GroupInput = {
  id: Scalars['String'],
  name: Scalars['String'],
  description: Scalars['String'],
};

export type GroupInvitation = {
   __typename?: 'GroupInvitation',
  id: Scalars['String'],
  group: Group,
  inviter: User,
};

export type GroupMember = {
   __typename?: 'GroupMember',
  user: User,
  role: GroupMemberRole,
};

export enum GroupMemberRole {
  Admin = 'ADMIN',
  Member = 'MEMBER'
}


export type InviteUsersInGroupInput = {
  id: Scalars['String'],
  usernames: Array<Scalars['String']>,
};

export type Invoice = {
   __typename?: 'Invoice',
  id: Scalars['String'],
};

export type Mutation = {
   __typename?: 'Mutation',
  register: RegistrationStarted,
  verifyRegistration: Scalars['Boolean'],
  sendNewRegistrationCode: Scalars['Boolean'],
  completeRegistration: SignedIn,
  signIn: SignedIn,
  revokeSession: Scalars['Boolean'],
  createGroup: Group,
  deleteGroup: Scalars['Boolean'],
  updateGroup: Group,
  removeGroupMembers: Group,
  inviteUsersInGroup: Group,
  acceptGroupInvitation: Scalars['Boolean'],
  declineGroupInvitation: Scalars['Boolean'],
  cancelGroupInvitation: Scalars['Boolean'],
  quitGroup: Scalars['Boolean'],
  createBillingPlan: BillingPlan,
  updateBillingPlan: BillingPlan,
  deleteBillingPlan: Scalars['Boolean'],
  changeBillingPlan: BillingPlan,
  addPaymentMethod?: Maybe<PaymentMethod>,
  removePaymentMethod: Scalars['Boolean'],
  changeDefaultPaymentMethod: PaymentMethod,
};


export type MutationRegisterArgs = {
  input: RegisterInput
};


export type MutationVerifyRegistrationArgs = {
  input: VerifyRegistrationInput
};


export type MutationSendNewRegistrationCodeArgs = {
  input: SendNewRegistrationCodeInput
};


export type MutationCompleteRegistrationArgs = {
  input: CompleteRegistrationInput
};


export type MutationSignInArgs = {
  input: SignInInput
};


export type MutationRevokeSessionArgs = {
  input: RevokeSessionInput
};


export type MutationCreateGroupArgs = {
  input: CreateGroupInput
};


export type MutationDeleteGroupArgs = {
  input: DeleteGroupInput
};


export type MutationUpdateGroupArgs = {
  input: GroupInput
};


export type MutationRemoveGroupMembersArgs = {
  input: RemoveGroupMembersInput
};


export type MutationInviteUsersInGroupArgs = {
  input: InviteUsersInGroupInput
};


export type MutationAcceptGroupInvitationArgs = {
  input: AcceptGroupInvitationInput
};


export type MutationDeclineGroupInvitationArgs = {
  input: DeclineGroupInvitationInput
};


export type MutationCancelGroupInvitationArgs = {
  input: CancelGroupInvitationInput
};


export type MutationQuitGroupArgs = {
  input: QuitGroupInput
};


export type MutationCreateBillingPlanArgs = {
  input: BillingPlanInput
};


export type MutationUpdateBillingPlanArgs = {
  input: BillingPlanInput
};


export type MutationDeleteBillingPlanArgs = {
  input: DeleteBillingPlanInput
};


export type MutationChangeBillingPlanArgs = {
  input: ChangeBillingPlanInput
};


export type MutationAddPaymentMethodArgs = {
  input: AddPaymentMethodInput
};


export type MutationRemovePaymentMethodArgs = {
  input: RemovePaymentMethodInput
};


export type MutationChangeDefaultPaymentMethodArgs = {
  input: ChangeDefaultPaymentMethodInput
};

export type PaymentMethod = {
   __typename?: 'PaymentMethod',
  id: Scalars['String'],
  createdAt: Scalars['Time'],
  cardLast4: Scalars['String'],
  cardExpirationMonth: Scalars['Int'],
  cardExpirationYear: Scalars['Int'],
};

export type Query = {
   __typename?: 'Query',
  me: User,
  user?: Maybe<User>,
  users: Array<User>,
  group?: Maybe<Group>,
  groups: Array<Group>,
  billingPlans: Array<BillingPlan>,
};


export type QueryUserArgs = {
  username: Scalars['String']
};


export type QueryUsersArgs = {
  limit?: Maybe<Scalars['Int']>,
  offset?: Maybe<Scalars['Int']>
};


export type QueryGroupArgs = {
  id: Scalars['String']
};


export type QueryGroupsArgs = {
  limit?: Maybe<Scalars['Int']>,
  offset?: Maybe<Scalars['Int']>
};

export type QuitGroupInput = {
  id: Scalars['String'],
};

export type RegisterInput = {
  displayName: Scalars['String'],
  email: Scalars['String'],
};

export type RegistrationStarted = {
   __typename?: 'RegistrationStarted',
  id: Scalars['String'],
};

export type RemoveGroupMembersInput = {
  id: Scalars['String'],
  usernames: Array<Scalars['String']>,
};

export type RemovePaymentMethodInput = {
  id: Scalars['String'],
};

export type RevokeSessionInput = {
  id: Scalars['String'],
};

export type SendNewRegistrationCodeInput = {
  id: Scalars['String'],
};

export type Session = {
   __typename?: 'Session',
  id: Scalars['String'],
  createdAt: Scalars['Time'],
  token?: Maybe<Scalars['String']>,
  device: SessionDevice,
};

export type SessionDevice = {
   __typename?: 'SessionDevice',
  os: SessionDeviceOs,
  type: SessionDeviceType,
};

export type SessionDeviceInput = {
  os: SessionDeviceOs,
  type: SessionDeviceType,
};

export enum SessionDeviceOs {
  Linux = 'LINUX',
  Macos = 'MACOS',
  Windows = 'WINDOWS',
  Android = 'ANDROID',
  Ios = 'IOS'
}

export enum SessionDeviceType {
  Tv = 'TV',
  Console = 'CONSOLE',
  Mobile = 'MOBILE',
  Tablet = 'TABLET',
  Watch = 'WATCH',
  Computer = 'COMPUTER',
  Car = 'CAR'
}

export type SignedIn = {
   __typename?: 'SignedIn',
  session: Session,
  me: User,
};

export type SignInInput = {
  username: Scalars['String'],
  authKey: Scalars['Bytes'],
  device: SessionDeviceInput,
};


export type User = {
   __typename?: 'User',
  id?: Maybe<Scalars['String']>,
  createdAt?: Maybe<Scalars['Time']>,
  username: Scalars['String'],
  firstName?: Maybe<Scalars['String']>,
  lastName?: Maybe<Scalars['String']>,
  displayName: Scalars['String'],
  isAdmin: Scalars['Boolean'],
  groups?: Maybe<Array<Group>>,
  paymentMethods?: Maybe<Array<PaymentMethod>>,
  invoices?: Maybe<Array<Invoice>>,
  sessions?: Maybe<Array<Session>>,
  groupInvitations?: Maybe<Array<GroupInvitation>>,
  stripePublicKey?: Maybe<Scalars['String']>,
  billingPlan?: Maybe<BillingPlan>,
};

export type VerifyRegistrationInput = {
  id: Scalars['String'],
  code: Scalars['String'],
};
