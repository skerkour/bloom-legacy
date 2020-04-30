const Index = () => import(/* webpackChunkName: "chunk-calendar" */ './views/calendar.vue');

export default [
  {
    component: Index,
    path: '/calendar',
  },
];
