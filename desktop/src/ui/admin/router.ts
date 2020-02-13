const Dashboard = () => import(/* webpackChunkName: "chunk-admin" */ './views/dashboard.vue');
const Users = () => import(/* webpackChunkName: "chunk-admin" */ './views/users.vue');
const Plans = () => import(/* webpackChunkName: "chunk-admin" */ './views/plans.vue');
const Groups = () => import(/* webpackChunkName: "chunk-admin" */ './views/groups.vue');

export default [
  {
    component: Dashboard,
    path: '/admin',
  },
  {
    component: Users,
    path: '/admin/users',
  },
  {
    component: Groups,
    path: '/admin/groups',
  },
  {
    component: Plans,
    path: '/admin/plans',
  },
];
