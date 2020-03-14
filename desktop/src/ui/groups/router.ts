const Groups = () => import(/* webpackChunkName: "chunk-groups" */ './views/groups.vue');
const Members = () => import(/* webpackChunkName: "chunk-groups" */ './views/members.vue');
const Preferences = () => import(/* webpackChunkName: "chunk-groups" */ './views/preferences.vue');
const Billing = () => import(/* webpackChunkName: "chunk-groups" */ './views/billing.vue');

export default [
  {
    component: Groups,
    path: '/groups',
  },
  {
    component: Members,
    path: '/groups/:groupId/members',
  },
  {
    component: Preferences,
    path: '/groups/:groupId/preferences',
  },
  {
    component: Billing,
    path: '/groups/:groupId/billing',
  },
];
