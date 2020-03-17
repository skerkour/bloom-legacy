import { Maybe, BloomMetadata, UserConnection } from '@/api/models';

/* eslint-disable import/prefer-default-export */
export enum Method {
  FetchDashBoardData = 'admin.fetchDashboardData',
}

export type DashboardData = {
  metadata: Maybe<BloomMetadata>,
  users: Maybe<UserConnection>,
};
