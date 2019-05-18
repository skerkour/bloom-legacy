const IndexView = () => import(/* webpackChunkName: "chunk-arcade" */ './views/Index.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
      },
      layout: 'unauthenticated',
      service: 'arcade',
    },
    name: 'arcade/index',
    path: '/arcade',
  },
];
