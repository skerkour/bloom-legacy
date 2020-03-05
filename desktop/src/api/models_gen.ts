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
  /** group id */
  id: Scalars['ID'],
};

export type AddPaymentMethodInput = {
  stripeId: Scalars['String'],
  /** if groupId is null, add to current user */
  groupId?: Maybe<Scalars['String']>,
};

export type BillingPlan = {
   __typename?: 'BillingPlan',
  id: Scalars['ID'],
  /** amount to pay in cents */
  price: Scalars['Int64'],
  name: Scalars['String'],
  /** plan's description, in HTML  */
  description: Scalars['String'],
  isPublic: Scalars['Boolean'],
  product: BillingProduct,
  storage: Scalars['Int64'],
  stripeId?: Maybe<Scalars['String']>,
  subscribers?: Maybe<UserConnection>,
};

export type BillingPlanConnection = {
   __typename?: 'BillingPlanConnection',
  edges?: Maybe<Array<Maybe<BillingPlanEdge>>>,
  pageInfo: PageInfo,
  totalCount: Scalars['Int64'],
};

export type BillingPlanEdge = {
   __typename?: 'BillingPlanEdge',
  cursor: Scalars['String'],
  node?: Maybe<BillingPlan>,
};

export type BillingPlanInput = {
  id?: Maybe<Scalars['ID']>,
  name: Scalars['String'],
  product: BillingProduct,
  /** the strip id of the stripe plan. starting with 'plan_' */
  stripeId: Scalars['String'],
  /** HTML description */
  description: Scalars['String'],
  isPublic: Scalars['Boolean'],
  storage: Scalars['Int64'],
};

export enum BillingProduct {
  Free = 'FREE',
  Lite = 'LITE',
  Pro = 'PRO',
  Ultra = 'ULTRA'
}

export type BillingSubscription = {
   __typename?: 'BillingSubscription',
  updatedAt: Scalars['Time'],
  usedStorage: Scalars['Int64'],
  stripeCustomerId?: Maybe<Scalars['String']>,
  stripeSubscriptionId?: Maybe<Scalars['String']>,
  plan: BillingPlan,
};

export type BloomMetadata = {
   __typename?: 'BloomMetadata',
  os: Scalars['String'],
  arch: Scalars['String'],
  version: Scalars['String'],
  gitCommit: Scalars['String'],
};


export type CancelGroupInvitationInput = {
  /** group id */
  id: Scalars['ID'],
};

/** set payment method with `id` as the default one */
export type ChangeDefaultPaymentMethodInput = {
  id: Scalars['ID'],
};

export type CompleteRegistrationInput = {
  /** pending user id */
  id: Scalars['ID'],
  username: Scalars['String'],
  authKey: Scalars['Bytes'],
  device: SessionDeviceInput,
};

export type CreateGroupInput = {
  name: Scalars['String'],
  description: Scalars['String'],
  /** users to invite, by username */
  usersToInvite: Array<Scalars['String']>,
};

export type DeclineGroupInvitationInput = {
  /** group id */
  id: Scalars['ID'],
};

export type DeleteBillingPlanInput = {
  id: Scalars['ID'],
};

export type DeleteGroupInput = {
  id: Scalars['ID'],
};

export type Group = {
   __typename?: 'Group',
  id?: Maybe<Scalars['ID']>,
  createdAt?: Maybe<Scalars['Time']>,
  avatarUrl?: Maybe<Scalars['String']>,
  name: Scalars['String'],
  description: Scalars['String'],
  members?: Maybe<GroupMemberConnection>,
  invitations?: Maybe<GroupInvitationConnection>,
  subscription?: Maybe<BillingSubscription>,
};

export type GroupConnection = {
   __typename?: 'GroupConnection',
  edges?: Maybe<Array<Maybe<GroupEdge>>>,
  pageInfo: PageInfo,
  totalCount: Scalars['Int64'],
};

export type GroupEdge = {
   __typename?: 'GroupEdge',
  cursor: Scalars['String'],
  node?: Maybe<Group>,
};

export type GroupInput = {
  /** group id */
  id: Scalars['ID'],
  name?: Maybe<Scalars['String']>,
  description?: Maybe<Scalars['String']>,
};

export type GroupInvitation = {
   __typename?: 'GroupInvitation',
  id: Scalars['ID'],
  group: Group,
  inviter: User,
  invitee: User,
};

export type GroupInvitationConnection = {
   __typename?: 'GroupInvitationConnection',
  edges?: Maybe<Array<Maybe<GroupInvitationEdge>>>,
  pageInfo: PageInfo,
  totalCount: Scalars['Int64'],
};

export type GroupInvitationEdge = {
   __typename?: 'GroupInvitationEdge',
  cursor: Scalars['String'],
  node?: Maybe<GroupInvitation>,
};

export type GroupMemberConnection = {
   __typename?: 'GroupMemberConnection',
  edges?: Maybe<Array<Maybe<GroupMemberEdge>>>,
  pageInfo: PageInfo,
  totalCount: Scalars['Int64'],
};

export type GroupMemberEdge = {
   __typename?: 'GroupMemberEdge',
  cursor: Scalars['String'],
  node?: Maybe<User>,
  role?: Maybe<GroupMemberRole>,
  joinedAt?: Maybe<Scalars['Time']>,
};

export enum GroupMemberRole {
  Admin = 'ADMIN',
  Member = 'MEMBER'
}


export type InviteUsersInGroupInput = {
  /** group id */
  id: Scalars['ID'],
  /** users to invite, by username */
  users: Array<Scalars['String']>,
};

export type Invoice = {
   __typename?: 'Invoice',
  id: Scalars['ID'],
  createdAt: Scalars['Time'],
  amount: Scalars['Int64'],
  stripeId: Scalars['String'],
  stripeHostedUrl: Scalars['String'],
  stripePdfUrl: Scalars['String'],
  paid: Scalars['Boolean'],
};

export type InvoiceConnection = {
   __typename?: 'InvoiceConnection',
  edges?: Maybe<Array<Maybe<InvoiceEdge>>>,
  pageInfo: PageInfo,
  totalCount: Scalars['Int64'],
};

export type InvoiceEdge = {
   __typename?: 'InvoiceEdge',
  cursor: Scalars['String'],
  node?: Maybe<Invoice>,
};

export type Mutation = {
   __typename?: 'Mutation',
  /** Start registration */
  register: RegistrationStarted,
  /** Verify pending account */
  verifyRegistration: Scalars['Boolean'],
  sendNewRegistrationCode: Scalars['Boolean'],
  /** Complete registration and create account */
  completeRegistration: SignedIn,
  /** Sign in */
  signIn: SignedIn,
  /** Revoke a session. Use it for sign out. */
  revokeSession: Scalars['Boolean'],
  /** Update an user profile, both private and public information */
  updateUserProfile: User,
  /** Create a group */
  createGroup: Group,
  /** Delete a group */
  deleteGroup: Scalars['Boolean'],
  /** Update a group information */
  updateGroup: Group,
  /** Remove users from a group */
  removeGroupMembers: Group,
  /** Invite users in a group */
  inviteUsersInGroup: Group,
  /** Accept a group invitaiton and join it */
  acceptGroupInvitation: Scalars['Boolean'],
  /** Decline a group invitation */
  declineGroupInvitation: Scalars['Boolean'],
  /** Cancel a group invitation */
  cancelGroupInvitation: Scalars['Boolean'],
  /** Quit a group */
  quitGroup: Scalars['Boolean'],
  createBillingPlan: BillingPlan,
  updateBillingPlan: BillingPlan,
  deleteBillingPlan: Scalars['Boolean'],
  updateBillingSubscription: BillingSubscription,
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


export type MutationUpdateUserProfileArgs = {
  input: UserProfileInput
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


export type MutationUpdateBillingSubscriptionArgs = {
  input: UpdateBillingSubscriptionInput
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

export type PageInfo = {
   __typename?: 'PageInfo',
  endCursor?: Maybe<Scalars['String']>,
  hasNextPage: Scalars['Boolean'],
  hasPreviousPage: Scalars['Boolean'],
  startCursor?: Maybe<Scalars['String']>,
};

export type PaymentMethod = {
   __typename?: 'PaymentMethod',
  id: Scalars['ID'],
  createdAt: Scalars['Time'],
  cardLast4: Scalars['String'],
  cardExpirationMonth: Scalars['Int'],
  cardExpirationYear: Scalars['Int'],
  isDefault: Scalars['Boolean'],
};

export type PaymentMethodConnection = {
   __typename?: 'PaymentMethodConnection',
  edges?: Maybe<Array<Maybe<PaymentMethodEdge>>>,
  pageInfo: PageInfo,
  totalCount: Scalars['Int64'],
};

export type PaymentMethodEdge = {
   __typename?: 'PaymentMethodEdge',
  cursor: Scalars['String'],
  node?: Maybe<PaymentMethod>,
};

export type Query = {
   __typename?: 'Query',
  /** Get information about current user */
  me: User,
  /** Find an user */
  user?: Maybe<User>,
  /** Find all users */
  users?: Maybe<UserConnection>,
  /** Find a group */
  group?: Maybe<Group>,
  /** Find all users */
  groups?: Maybe<GroupConnection>,
  /** Find all billing plans visible to the current user */
  billingPlans?: Maybe<BillingPlanConnection>,
  /** Metadata about Bloom server */
  metadata?: Maybe<BloomMetadata>,
  /** The stripe public key to be used */
  stripePublicKey: Scalars['String'],
};


export type QueryUserArgs = {
  username?: Maybe<Scalars['String']>
};


export type QueryGroupArgs = {
  id: Scalars['ID']
};

export type QuitGroupInput = {
  /** group id */
  id: Scalars['ID'],
};

export type RegisterInput = {
  displayName: Scalars['String'],
  email: Scalars['String'],
};

export type RegistrationStarted = {
   __typename?: 'RegistrationStarted',
  id: Scalars['ID'],
};

export type RemoveGroupMembersInput = {
  /** group id */
  id: Scalars['ID'],
  /** members to remvove, by username */
  members: Array<Scalars['String']>,
};

/** remove payment method with `id` */
export type RemovePaymentMethodInput = {
  id: Scalars['ID'],
};

export type RevokeSessionInput = {
  id: Scalars['ID'],
};

export type SendNewRegistrationCodeInput = {
  id: Scalars['ID'],
};

export type Session = {
   __typename?: 'Session',
  id: Scalars['ID'],
  createdAt: Scalars['Time'],
  token?: Maybe<Scalars['String']>,
  device: SessionDevice,
};

export type SessionConnection = {
   __typename?: 'SessionConnection',
  edges?: Maybe<Array<Maybe<SessionEdge>>>,
  pageInfo: PageInfo,
  totalCount: Scalars['Int64'],
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
  Ios = 'IOS',
  Other = 'OTHER'
}

export enum SessionDeviceType {
  Tv = 'TV',
  Console = 'CONSOLE',
  Mobile = 'MOBILE',
  Tablet = 'TABLET',
  Watch = 'WATCH',
  Computer = 'COMPUTER',
  Car = 'CAR',
  Other = 'OTHER'
}

export type SessionEdge = {
   __typename?: 'SessionEdge',
  cursor: Scalars['String'],
  node?: Maybe<Session>,
};

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


/** if groupId and userId are null (reserved for admins), add to current user */
export type UpdateBillingSubscriptionInput = {
  planId: Scalars['ID'],
  userId?: Maybe<Scalars['String']>,
  groupId?: Maybe<Scalars['String']>,
};

export type User = {
   __typename?: 'User',
  id?: Maybe<Scalars['ID']>,
  createdAt?: Maybe<Scalars['Time']>,
  avatarUrl?: Maybe<Scalars['String']>,
  username: Scalars['String'],
  firstName?: Maybe<Scalars['String']>,
  lastName?: Maybe<Scalars['String']>,
  email?: Maybe<Scalars['String']>,
  displayName: Scalars['String'],
  bio: Scalars['String'],
  isAdmin: Scalars['Boolean'],
  groups?: Maybe<GroupConnection>,
  paymentMethods?: Maybe<PaymentMethodConnection>,
  invoices?: Maybe<InvoiceConnection>,
  sessions?: Maybe<SessionConnection>,
  groupInvitations?: Maybe<GroupInvitationConnection>,
  subscription?: Maybe<BillingSubscription>,
};

export type UserConnection = {
   __typename?: 'UserConnection',
  edges?: Maybe<Array<Maybe<UserEdge>>>,
  pageInfo: PageInfo,
  totalCount: Scalars['Int64'],
};

export type UserEdge = {
   __typename?: 'UserEdge',
  cursor: Scalars['String'],
  node?: Maybe<User>,
};

export type UserProfileInput = {
  /** id is reserved for admins */
  id?: Maybe<Scalars['ID']>,
  displayName?: Maybe<Scalars['String']>,
  bio?: Maybe<Scalars['String']>,
  firstName?: Maybe<Scalars['String']>,
  lastName?: Maybe<Scalars['String']>,
};

export type VerifyRegistrationInput = {
  /** pending user id */
  id: Scalars['ID'],
  code: Scalars['String'],
};
