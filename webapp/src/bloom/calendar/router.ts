const IndexView = () => import(/* webpackChunkName: "chunk-calendar" */ './views/Index.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
      },
      layout: 'unauthenticated',
      service: 'calendar',
    },
    name: 'calendar/index',
    path: '/calendar',
  },
];
