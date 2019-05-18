const IndexView = () => import(/* webpackChunkName: "chunk-contacts" */ './views/Index.vue'); // tslint:disable-line

export default [
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
      },
      layout: 'unauthenticated',
      service: 'contacts',
    },
    path: '/contacts',
  },
];
