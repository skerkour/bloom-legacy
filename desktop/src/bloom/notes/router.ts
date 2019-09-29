const Index = () => import(/* webpackChunkName: "chunk-notes" */ './views/Index.vue');

export default [
  {
    component: Index,
    path: '/notes',
  },
];
