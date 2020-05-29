const Sync = () => import(/* webpackChunkName: "chunk-kernel" */ './views/sync.vue');

export default [
  {
    component: Sync,
    path: '/sync',
  },
];
