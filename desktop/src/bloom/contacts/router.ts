const Index = () => import(/* webpackChunkName: "chunk-contacts" */ './views/Index.vue');

export default [
  {
    component: Index,
    path: '/contacts',
  },
];
