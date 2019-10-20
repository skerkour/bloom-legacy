const Index = () => import(/* webpackChunkName: "chunk-gallery" */ './views/Index.vue');

export default [
  {
    component: Index,
    path: '/gallery',
  },
];
