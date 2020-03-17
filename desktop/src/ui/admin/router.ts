const Dashboard = () => import(/* webpackChunkName: "chunk-admin" */ './views/dashboard.vue');
const Users = () => import(/* webpackChunkName: "chunk-admin" */ './views/users.vue');
const Plans = () => import(/* webpackChunkName: "chunk-admin" */ './views/plans.vue');
const Groups = () => import(/* webpackChunkName: "chunk-admin" */ './views/groups.vue');
const User = () => import(/* webpackChunkName: "chunk-admin" */ './views/user.vue');
const Group = () => import(/* webpackChunkName: "chunk-admin" */ './views/group.vue');

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
    component: User,
    path: '/admin/users/:username',
  },
  {
    component: Groups,
    path: '/admin/groups',
  },
  {
    component: Group,
    path: '/admin/groups/:groupId',
  },
  {
    component: Plans,
    path: '/admin/plans',
  },
];
