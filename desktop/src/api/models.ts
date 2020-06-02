export type Maybe<T> = T | null;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  Time: any;
  Bytes: any;
  Int64: any;
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

export enum GroupMemberRole {
  Admin = 'ADMIN',
  Member = 'MEMBER'
}

export enum BillingProduct {
  Free = 'FREE',
  Lite = 'LITE',
  Pro = 'PRO',
  Ultra = 'ULTRA'
}

export enum TwoFaMethod {
  Totp = 'TOTP'
}

export type PageInfo = {
  __typename?: 'PageInfo';
  endCursor?: Maybe<Scalars['String']>;
  hasNextPage: Scalars['Boolean'];
  hasPreviousPage: Scalars['Boolean'];
  startCursor?: Maybe<Scalars['String']>;
};

export type User = {
  __typename?: 'User';
  id?: Maybe<Scalars['ID']>;
  createdAt?: Maybe<Scalars['Time']>;
  avatarUrl?: Maybe<Scalars['String']>;
  username: Scalars['String'];
  firstName?: Maybe<Scalars['String']>;
  lastName?: Maybe<Scalars['String']>;
  email?: Maybe<Scalars['String']>;
  displayName: Scalars['String'];
  bio: Scalars['String'];
  isAdmin: Scalars['Boolean'];
  disabledAt?: Maybe<Scalars['Time']>;
  state?: Maybe<Scalars['String']>;
  publicKey: Scalars['Bytes'];
  encryptedPrivateKey?: Maybe<Scalars['Bytes']>;
  privateKeyNonce?: Maybe<Scalars['Bytes']>;
  encryptedMasterKey?: Maybe<Scalars['Bytes']>;
  masterKeyNonce?: Maybe<Scalars['Bytes']>;
  groups?: Maybe<GroupConnection>;
  paymentMethods?: Maybe<PaymentMethodConnection>;
  invoices?: Maybe<InvoiceConnection>;
  sessions?: Maybe<SessionConnection>;
  groupInvitations?: Maybe<GroupInvitationConnection>;
  subscription?: Maybe<BillingSubscription>;
};

export type UserConnection = {
  __typename?: 'UserConnection';
  nodes: Array<User>;
  pageInfo: PageInfo;
  totalCount: Scalars['Int64'];
};

export type PaymentMethod = {
  __typename?: 'PaymentMethod';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  cardLast4: Scalars['String'];
  cardExpirationMonth: Scalars['Int'];
  cardExpirationYear: Scalars['Int'];
  isDefault: Scalars['Boolean'];
};

export type PaymentMethodConnection = {
  __typename?: 'PaymentMethodConnection';
  nodes: Array<PaymentMethod>;
  pageInfo: PageInfo;
  totalCount: Scalars['Int64'];
};

export type Group = {
  __typename?: 'Group';
  id?: Maybe<Scalars['ID']>;
  createdAt?: Maybe<Scalars['Time']>;
  avatarUrl?: Maybe<Scalars['String']>;
  name: Scalars['String'];
  description: Scalars['String'];
  state?: Maybe<Scalars['String']>;
  encryptedMasterKey?: Maybe<Scalars['Bytes']>;
  masterKeyNonce?: Maybe<Scalars['Bytes']>;
  members?: Maybe<GroupMemberConnection>;
  invitations?: Maybe<GroupInvitationConnection>;
  subscription?: Maybe<BillingSubscription>;
  paymentMethods?: Maybe<PaymentMethodConnection>;
  invoices?: Maybe<InvoiceConnection>;
};

export type GroupConnection = {
  __typename?: 'GroupConnection';
  nodes: Array<Group>;
  pageInfo: PageInfo;
  totalCount: Scalars['Int64'];
};

export type GroupMemberConnection = {
  __typename?: 'GroupMemberConnection';
  edges?: Maybe<Array<Maybe<GroupMemberEdge>>>;
  pageInfo: PageInfo;
  totalCount: Scalars['Int64'];
};

export type GroupMemberEdge = {
  __typename?: 'GroupMemberEdge';
  node?: Maybe<User>;
  role?: Maybe<GroupMemberRole>;
  joinedAt?: Maybe<Scalars['Time']>;
};

export type GroupInvitation = {
  __typename?: 'GroupInvitation';
  id: Scalars['ID'];
  group: Group;
  inviter: User;
  invitee: User;
  ephemeralPublicKey?: Maybe<Scalars['Bytes']>;
  encryptedMasterKey?: Maybe<Scalars['Bytes']>;
  signature?: Maybe<Scalars['Bytes']>;
};

export type GroupInvitationConnection = {
  __typename?: 'GroupInvitationConnection';
  nodes: Array<GroupInvitation>;
  pageInfo: PageInfo;
  totalCount: Scalars['Int64'];
};

export type Invoice = {
  __typename?: 'Invoice';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  amount: Scalars['Int64'];
  stripeId: Scalars['String'];
  stripeHostedUrl: Scalars['String'];
  stripePdfUrl: Scalars['String'];
  paidAt?: Maybe<Scalars['Time']>;
};

export type InvoiceConnection = {
  __typename?: 'InvoiceConnection';
  nodes: Array<Invoice>;
  pageInfo: PageInfo;
  totalCount: Scalars['Int64'];
};

export type BillingPlan = {
  __typename?: 'BillingPlan';
  id: Scalars['ID'];
  /** amount to pay in cents */
  price: Scalars['Int64'];
  name: Scalars['String'];
  /** plan's description, in HTML  */
  description: Scalars['String'];
  isPublic: Scalars['Boolean'];
  product: BillingProduct;
  storage: Scalars['Int64'];
  stripeId?: Maybe<Scalars['String']>;
  subscribers?: Maybe<UserConnection>;
};

export type BillingPlanConnection = {
  __typename?: 'BillingPlanConnection';
  nodes: Array<BillingPlan>;
  pageInfo: PageInfo;
  totalCount: Scalars['Int64'];
};

export type BillingSubscription = {
  __typename?: 'BillingSubscription';
  updatedAt: Scalars['Time'];
  usedStorage: Scalars['Int64'];
  stripeCustomerId?: Maybe<Scalars['String']>;
  stripeSubscriptionId?: Maybe<Scalars['String']>;
  plan: BillingPlan;
};

export type Session = {
  __typename?: 'Session';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  token?: Maybe<Scalars['String']>;
  device: SessionDevice;
};

export type SessionConnection = {
  __typename?: 'SessionConnection';
  nodes: Array<Session>;
  pageInfo: PageInfo;
  totalCount: Scalars['Int64'];
};

export type SessionDevice = {
  __typename?: 'SessionDevice';
  os: SessionDeviceOs;
  type: SessionDeviceType;
};

export type RegistrationStarted = {
  __typename?: 'RegistrationStarted';
  id: Scalars['ID'];
};

export type SignedIn = {
  __typename?: 'SignedIn';
  pendingSession?: Maybe<PendingSession>;
  session: Session;
  me: User;
};

export type PendingSession = {
  __typename?: 'PendingSession';
  id: Scalars['ID'];
  token: Scalars['String'];
  twoFA?: Maybe<TwoFa>;
};

export type TwoFa = {
  __typename?: 'TwoFA';
  method: TwoFaMethod;
};

export type PasswordUpdateStarted = {
  __typename?: 'PasswordUpdateStarted';
  /** pending_password id */
  id: Scalars['ID'];
  twoFA?: Maybe<TwoFa>;
};

export type Push = {
  __typename?: 'Push';
  repositories: Array<RepositoryPush>;
};

export type RepositoryPush = {
  __typename?: 'RepositoryPush';
  oldState: Scalars['String'];
  newState: Scalars['String'];
  groupID?: Maybe<Scalars['ID']>;
};

export type Pull = {
  __typename?: 'Pull';
  repositories: Array<RepositoryPull>;
};

export type RepositoryPull = {
  __typename?: 'RepositoryPull';
  oldState: Scalars['String'];
  newState: Scalars['String'];
  objects: Array<Object>;
  hasMoreChanges: Scalars['Boolean'];
  groupID?: Maybe<Scalars['ID']>;
};

export type Object = {
  __typename?: 'Object';
  id: Scalars['Bytes'];
  algorithm: Scalars['String'];
  encryptedData: Scalars['Bytes'];
  encryptedKey: Scalars['Bytes'];
  nonce: Scalars['Bytes'];
};

export type TwoFaActivationStarted = {
  __typename?: 'TwoFAActivationStarted';
  qrCode: Scalars['Bytes'];
};

export type BloomMetadata = {
  __typename?: 'BloomMetadata';
  os: Scalars['String'];
  arch: Scalars['String'];
  version: Scalars['String'];
  gitCommit: Scalars['String'];
};

export type Query = {
  __typename?: 'Query';
  /** Get information about current user */
  me: User;
  /** Find an user */
  user?: Maybe<User>;
  /** Find all users */
  users?: Maybe<UserConnection>;
  /** Find a group */
  group?: Maybe<Group>;
  /** Find all groups */
  groups?: Maybe<GroupConnection>;
  /** Find all billing plans visible to the current user */
  billingPlans?: Maybe<BillingPlanConnection>;
  /** Metadata about Bloom server */
  metadata?: Maybe<BloomMetadata>;
  /** The stripe public key to be used */
  stripePublicKey: Scalars['String'];
  /** Pull changes since a given state */
  pull: Pull;
};


export type QueryUserArgs = {
  username?: Maybe<Scalars['String']>;
};


export type QueryGroupArgs = {
  id: Scalars['ID'];
};


export type QueryPullArgs = {
  input: PullInput;
};

export type StartRegistrationInput = {
  displayName: Scalars['String'];
  email: Scalars['String'];
};

export type VerifyUserInput = {
  /** pending user id */
  id: Scalars['ID'];
  code: Scalars['String'];
};

export type CompleteRegistrationInput = {
  /** pending user id */
  id: Scalars['ID'];
  username: Scalars['String'];
  authKey: Scalars['Bytes'];
  device: SessionDeviceInput;
  publicKey: Scalars['Bytes'];
  encryptedPrivateKey: Scalars['Bytes'];
  privateKeyNonce: Scalars['Bytes'];
  encryptedMasterKey: Scalars['Bytes'];
  masterKeyNonce: Scalars['Bytes'];
};

export type SessionDeviceInput = {
  os: SessionDeviceOs;
  type: SessionDeviceType;
};

export type SignInInput = {
  username: Scalars['String'];
  authKey: Scalars['Bytes'];
  device: SessionDeviceInput;
};

export type CompleteSignInInput = {
  /** pending_session id */
  id: Scalars['ID'];
  token: Scalars['String'];
  twoFACode: Scalars['String'];
};

export type RevokeSessionInput = {
  id: Scalars['ID'];
};

export type SendNewRegistrationCodeInput = {
  id: Scalars['ID'];
};

export type CreateGroupInput = {
  name: Scalars['String'];
  description: Scalars['String'];
  encryptedMasterKey: Scalars['Bytes'];
  masterKeyNonce: Scalars['Bytes'];
};

export type DeleteGroupInput = {
  id: Scalars['ID'];
};

export type GroupInput = {
  /** group id */
  id: Scalars['ID'];
  name?: Maybe<Scalars['String']>;
  description?: Maybe<Scalars['String']>;
};

export type RemoveGroupMembersInput = {
  groupID: Scalars['ID'];
  /** members to remvove, by username */
  members: Array<Scalars['String']>;
};

export type AcceptGroupInvitationInput = {
  invitationID: Scalars['ID'];
  encryptedMasterKey: Scalars['Bytes'];
  masterKeyNonce: Scalars['Bytes'];
};

export type CancelGroupInvitationInput = {
  invitationID: Scalars['ID'];
};

export type DeclineGroupInvitationInput = {
  invitationID: Scalars['ID'];
};

export type InviteUserInGroupInput = {
  groupID: Scalars['ID'];
  username: Scalars['String'];
  ephemeralPublicKey: Scalars['Bytes'];
  encryptedMasterKey: Scalars['Bytes'];
  signature: Scalars['Bytes'];
};

export type QuitGroupInput = {
  groupID: Scalars['ID'];
};

export type BillingPlanInput = {
  id?: Maybe<Scalars['ID']>;
  name: Scalars['String'];
  product: BillingProduct;
  /** the strip id of the stripe plan. starting with 'plan_' */
  stripeId: Scalars['String'];
  /** HTML description */
  description: Scalars['String'];
  isPublic: Scalars['Boolean'];
  storage: Scalars['Int64'];
};

export type DeleteBillingPlanInput = {
  id: Scalars['ID'];
};

/** if groupID and userId (reserved for admins) are null, add to current user */
export type UpdateBillingSubscriptionInput = {
  planId: Scalars['ID'];
  userId?: Maybe<Scalars['ID']>;
  groupID?: Maybe<Scalars['ID']>;
};

export type AddPaymentMethodInput = {
  stripeId: Scalars['String'];
  /** if groupID is null, add to current user */
  groupID?: Maybe<Scalars['ID']>;
};

/** remove payment method with `id` */
export type RemovePaymentMethodInput = {
  id: Scalars['ID'];
};

/** set payment method with `id` as the default one */
export type ChangeDefaultPaymentMethodInput = {
  id: Scalars['ID'];
};

export type UserProfileInput = {
  /** id is reserved for admins */
  id?: Maybe<Scalars['ID']>;
  displayName?: Maybe<Scalars['String']>;
  bio?: Maybe<Scalars['String']>;
  firstName?: Maybe<Scalars['String']>;
  lastName?: Maybe<Scalars['String']>;
};

export type VerifyPasswordUpdateInput = {
  id: Scalars['ID'];
  code: Scalars['String'];
  twoFACode?: Maybe<Scalars['String']>;
};

export type CompletePasswordUpdateInput = {
  id: Scalars['ID'];
  authKey: Scalars['Bytes'];
  device: SessionDeviceInput;
  encryptedMasterKey: Scalars['Bytes'];
  masterKeyNonce: Scalars['Bytes'];
};

export type PullInput = {
  repositories: Array<RepositoryPullInput>;
};

export type RepositoryPullInput = {
  sinceState: Scalars['String'];
  groupID?: Maybe<Scalars['ID']>;
};

export type PushInput = {
  repositories: Array<RepositoryPushInput>;
};

export type RepositoryPushInput = {
  /** current state of the client */
  currentState: Scalars['String'];
  /** out of sync objects */
  objects: Array<ObjectInput>;
  /** to indicate whether it's the user's repository, or a group */
  groupID?: Maybe<Scalars['ID']>;
};

export type ObjectInput = {
  id: Scalars['Bytes'];
  algorithm: Scalars['String'];
  encryptedData: Scalars['Bytes'];
  encryptedKey: Scalars['Bytes'];
  nonce: Scalars['Bytes'];
};

export type DisableTwoFaInput = {
  code: Scalars['String'];
};

export type CompleteTwoFaActivationInput = {
  code: Scalars['String'];
};

export type Mutation = {
  __typename?: 'Mutation';
  startRegistration: RegistrationStarted;
  /** Verify pending account */
  verifyUser: Scalars['Boolean'];
  sendNewRegistrationCode: Scalars['Boolean'];
  /** Complete registration and create account */
  completeRegistration: SignedIn;
  /** Sign in */
  signIn?: Maybe<SignedIn>;
  completeSignIn?: Maybe<SignedIn>;
  /** Revoke a session. Use it for sign out. */
  revokeSession: Scalars['Boolean'];
  /** Update an user profile, both private and public information */
  updateUserProfile: User;
  disableUser: Scalars['Boolean'];
  enableUser: Scalars['Boolean'];
  /** Update password */
  startPasswordUpdate: PasswordUpdateStarted;
  verifyPasswordUpdate: Scalars['Boolean'];
  completePasswordUpdate: SignedIn;
  /** 2fa */
  startTwoFAActivation?: Maybe<TwoFaActivationStarted>;
  completeTwoFAActivation: Scalars['Boolean'];
  disableTwoFA: Scalars['Boolean'];
  /** Create a group */
  createGroup: Group;
  /** Delete a group */
  deleteGroup: Scalars['Boolean'];
  /** Update a group information */
  updateGroup: Group;
  /** Remove users from a group */
  removeGroupMembers: Group;
  /** Invite users in a group */
  inviteUserInGroup: Group;
  /** Accept a group invitaiton and join it */
  acceptGroupInvitation: Group;
  /** Decline a group invitation */
  declineGroupInvitation: Scalars['Boolean'];
  /** Cancel a group invitation */
  cancelGroupInvitation: Scalars['Boolean'];
  /** Quit a group */
  quitGroup: Scalars['Boolean'];
  createBillingPlan: BillingPlan;
  updateBillingPlan: BillingPlan;
  deleteBillingPlan: Scalars['Boolean'];
  updateBillingSubscription: BillingSubscription;
  addPaymentMethod?: Maybe<PaymentMethod>;
  removePaymentMethod: Scalars['Boolean'];
  changeDefaultPaymentMethod: PaymentMethod;
  push: Push;
};


export type MutationStartRegistrationArgs = {
  input: StartRegistrationInput;
};


export type MutationVerifyUserArgs = {
  input: VerifyUserInput;
};


export type MutationSendNewRegistrationCodeArgs = {
  input: SendNewRegistrationCodeInput;
};


export type MutationCompleteRegistrationArgs = {
  input: CompleteRegistrationInput;
};


export type MutationSignInArgs = {
  input: SignInInput;
};


export type MutationCompleteSignInArgs = {
  input: CompleteSignInInput;
};


export type MutationRevokeSessionArgs = {
  input: RevokeSessionInput;
};


export type MutationUpdateUserProfileArgs = {
  input: UserProfileInput;
};


export type MutationDisableUserArgs = {
  id: Scalars['ID'];
};


export type MutationEnableUserArgs = {
  id: Scalars['ID'];
};


export type MutationVerifyPasswordUpdateArgs = {
  input: VerifyPasswordUpdateInput;
};


export type MutationCompletePasswordUpdateArgs = {
  input: CompletePasswordUpdateInput;
};


export type MutationCompleteTwoFaActivationArgs = {
  input: CompleteTwoFaActivationInput;
};


export type MutationDisableTwoFaArgs = {
  input: DisableTwoFaInput;
};


export type MutationCreateGroupArgs = {
  input: CreateGroupInput;
};


export type MutationDeleteGroupArgs = {
  input: DeleteGroupInput;
};


export type MutationUpdateGroupArgs = {
  input: GroupInput;
};


export type MutationRemoveGroupMembersArgs = {
  input: RemoveGroupMembersInput;
};


export type MutationInviteUserInGroupArgs = {
  input: InviteUserInGroupInput;
};


export type MutationAcceptGroupInvitationArgs = {
  input: AcceptGroupInvitationInput;
};


export type MutationDeclineGroupInvitationArgs = {
  input: DeclineGroupInvitationInput;
};


export type MutationCancelGroupInvitationArgs = {
  input: CancelGroupInvitationInput;
};


export type MutationQuitGroupArgs = {
  input: QuitGroupInput;
};


export type MutationCreateBillingPlanArgs = {
  input: BillingPlanInput;
};


export type MutationUpdateBillingPlanArgs = {
  input: BillingPlanInput;
};


export type MutationDeleteBillingPlanArgs = {
  input: DeleteBillingPlanInput;
};


export type MutationUpdateBillingSubscriptionArgs = {
  input: UpdateBillingSubscriptionInput;
};


export type MutationAddPaymentMethodArgs = {
  input: AddPaymentMethodInput;
};


export type MutationRemovePaymentMethodArgs = {
  input: RemovePaymentMethodInput;
};


export type MutationChangeDefaultPaymentMethodArgs = {
  input: ChangeDefaultPaymentMethodInput;
};


export type MutationPushArgs = {
  input: PushInput;
};
