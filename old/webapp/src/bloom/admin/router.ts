const AccountsIndexView = () => import(/* webpackChunkName: "chunk-admin" */ './views/accounts/Index.vue'); // tslint:disable-line
const AccountIndexView = () => import(/* webpackChunkName: "chunk-admin" */ './views/accounts/account/Index.vue'); // tslint:disable-line
const DashboardView = () => import(/* webpackChunkName: "chunk-admin" */ './views/dashboard/Index.vue'); // tslint:disable-line


export default [
  {
    component: DashboardView,
    meta: {
      auth: {
        required: true,
      },
      service: 'admin',
    },
    name: 'admin/dashboard',
    path: '/admin',
  },
  {
    component: AccountsIndexView,
    meta: {
      auth: {
        required: true,
      },
      service: 'admin',
    },
    name: 'admin/accounts',
    path: '/admin/accounts',
  },
  {
    component: AccountIndexView,
    meta: {
      auth: {
        required: true,
      },
      service: 'admin',
    },
    name: 'admin/account',
    path: '/admin/accounts/:account_id',
  },
];
