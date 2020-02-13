const Index = () => import(/* webpackChunkName: "chunk-chat" */ './views/Index.vue');

export default [
  {
    component: Index,
    path: '/chat',
  },
];
