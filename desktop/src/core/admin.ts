import { Maybe, BloomMetadata, UserConnection } from '@/api/models';

export enum Method {
  FetchDashBoardData = 'admin.fetchDashboardData',
}

export type DashboardData = {
  metadata: Maybe<BloomMetadata>,
  users: Maybe<UserConnection>,
};
