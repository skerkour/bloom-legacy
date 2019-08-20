const IndexView = () => import(/* webpackChunkName: "chunk-calendar" */ './views/Index.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      service: 'calendar',
    },
    name: 'calendar/index',
    path: '/calendar',
  },
];
