const Theme = () => import(/* webpackChunkName: "chunk-preferences" */ './views/theme.vue');

export default [
  {
    component: Theme,
    path: '/preferences',
  },
];
