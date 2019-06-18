const IndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/Index.vue'); // tslint:disable-line

export default [
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
      },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help',
  },
];
