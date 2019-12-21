const Downloads = () => import(/* webpackChunkName: "chunk-bitflow" */ './views/downloads.vue');
const History = () => import(/* webpackChunkName: "chunk-bitflow" */ './views/history.vue');

export default [
  {
    component: Downloads,
    path: '/bitflow',
  },
  {
    component: History,
    path: '/bitflow/history',
  },
];
