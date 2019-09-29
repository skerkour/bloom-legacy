const Index = () => import(/* webpackChunkName: "chunk-music" */ './views/Index.vue');

export default [
  {
    component: Index,
    path: '/music',
  },
];
