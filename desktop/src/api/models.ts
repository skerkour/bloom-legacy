import { ServerVersion, Maybe } from './models_gen';

export * from './models_gen';

export type DashboardData = {
  serverVersion: Maybe<ServerVersion>,
};
