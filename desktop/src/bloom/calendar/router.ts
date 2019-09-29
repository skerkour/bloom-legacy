const Index = () => import(/* webpackChunkName: "chunk-calendar" */ './views/Index.vue');

export default [
  {
    component: Index,
    path: '/calendar',
  },
];
