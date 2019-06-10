const IndexView = () => import(/* webpackChunkName: "chunk-admin" */ '../kernel/components/ComingSoon.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
        required: true,
      },
      service: 'admin',
    },
    name: 'admin/index',
    path: '/admin',
  },
];
