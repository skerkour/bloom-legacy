const AccountsIndexView = () => import(/* webpackChunkName: "chunk-admin" */ './views/accounts/Index.vue'); // tslint:disable-line


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
    name: 'admin/index',
    path: '/admin/accounts',
  },
  { path: '/admin', redirect: '/admin/accounts' },
];
