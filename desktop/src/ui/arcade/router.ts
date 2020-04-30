const Games = () => import(/* webpackChunkName: "chunk-arcade" */ './views/games.vue');

export default [
  {
    component: Games,
    path: '/arcade',
  },
];
