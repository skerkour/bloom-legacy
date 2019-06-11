const AccountsIndexView = () => import(/* webpackChunkName: "chunk-admin" */ './views/accounts/Index.vue'); // tslint:disable-line
const AccountIndexView = () => import(/* webpackChunkName: "chunk-admin" */ './views/accounts/account/Index.vue'); // tslint:disable-line


export default [
  {
    component: AccountsIndexView,
    meta: {
      auth: {
        layout: 'authenticated',
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
        layout: 'authenticated',
        required: true,
      },
      service: 'admin',
    },
    name: 'admin/account',
    path: '/admin/accounts/:account_id',
  },
  { path: '/admin', redirect: '/admin/accounts' },
];
