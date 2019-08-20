const IndexView = () => import(/* webpackChunkName: "chunk-contacts" */ './views/Index.vue'); // tslint:disable-line

export default [
  {
    component: IndexView,
    meta: {
      service: 'contacts',
    },
    path: '/contacts',
  },
];
