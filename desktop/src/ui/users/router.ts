const Sync = () => import(/* webpackChunkName: "chunk-users" */ './views/sync.vue');

export default [
  {
    component: Sync,
    path: '/sync',
  },
];
