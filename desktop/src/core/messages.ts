export type Maybe<T> = T | null;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  /**
   * This schema is only used to auto genrate the struct for the communication between the core and
   * the apps.
   */
  BloomMetadata: any;
  UserConnection: any;
  Group: any;
  BillingPlanConnection: any;
  User: any;
  Time: any;
  Bytes: any;
};


export type DashboardData = {
  __typename?: 'DashboardData';
  metadata?: Maybe<Scalars['BloomMetadata']>;
  users?: Maybe<Scalars['UserConnection']>;
};

export type FetchGroupProfileParams = {
  __typename?: 'FetchGroupProfileParams';
  id: Scalars['ID'];
};

export type GroupBillingProfile = {
  __typename?: 'GroupBillingProfile';
  group?: Maybe<Scalars['Group']>;
  billingPlans?: Maybe<Scalars['BillingPlanConnection']>;
  stripePublicKey?: Maybe<Scalars['String']>;
};

export type UserBillingProfile = {
  __typename?: 'UserBillingProfile';
  user?: Maybe<Scalars['User']>;
  billingPlans?: Maybe<Scalars['BillingPlanConnection']>;
};

export type NewStripeCard = {
  __typename?: 'NewStripeCard';
  number: Scalars['String'];
  expMonth: Scalars['String'];
  expYear: Scalars['String'];
  cvc: Scalars['String'];
};

export type AddPaymentMethodParams = {
  __typename?: 'AddPaymentMethodParams';
  stripePublicKey?: Maybe<Scalars['String']>;
  groupId?: Maybe<Scalars['ID']>;
  card: NewStripeCard;
};

export type MyBillingProfile = {
  __typename?: 'MyBillingProfile';
  me?: Maybe<Scalars['User']>;
  billingPlans?: Maybe<Scalars['BillingPlanConnection']>;
  stripePublicKey: Scalars['String'];
};

export type CalcParams = {
  __typename?: 'CalcParams';
  expression: Scalars['String'];
};

export type CalcResult = {
  __typename?: 'CalcResult';
  result: Scalars['String'];
};

export type CalendarListEventsParams = {
  __typename?: 'CalendarListEventsParams';
  startAt?: Maybe<Scalars['Time']>;
  endAt?: Maybe<Scalars['Time']>;
};

export type CalendarCreateEventParams = {
  __typename?: 'CalendarCreateEventParams';
  title: Scalars['String'];
  location: Scalars['String'];
  description: Scalars['String'];
  startAt: Scalars['Time'];
  endAt: Scalars['Time'];
};

export type CalendarDeleteEventParams = {
  __typename?: 'CalendarDeleteEventParams';
  id: Scalars['Bytes'];
};


export type DeleteContactParams = {
  __typename?: 'DeleteContactParams';
  id: Scalars['Bytes'];
};

export type FetchGroupMembersParams = {
  __typename?: 'FetchGroupMembersParams';
  groupID: Scalars['ID'];
};

export type FetchGroupDetailsParams = {
  __typename?: 'FetchGroupDetailsParams';
  groupID: Scalars['ID'];
};

export type InviteUserInGroupParams = {
  __typename?: 'InviteUserInGroupParams';
  groupID: Scalars['ID'];
  username: Scalars['String'];
};

export type Empty = {
  __typename?: 'Empty';
  noop?: Maybe<Scalars['Boolean']>;
};

export type CreateNoteParams = {
  __typename?: 'CreateNoteParams';
  title: Scalars['String'];
  body: Scalars['String'];
  color: Scalars['String'];
  groupId?: Maybe<Scalars['ID']>;
};

export type DeleteNoteParams = {
  __typename?: 'DeleteNoteParams';
  id: Scalars['Bytes'];
};

export type PreferencesSetParams = {
  __typename?: 'PreferencesSetParams';
  key: Scalars['String'];
  value: Scalars['String'];
};

export type PreferencesGetParams = {
  __typename?: 'PreferencesGetParams';
  key: Scalars['String'];
};

export type PreferencesDeleteParams = {
  __typename?: 'PreferencesDeleteParams';
  key: Scalars['String'];
};
