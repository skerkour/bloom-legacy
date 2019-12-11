const Dashboard = () => import(/* webpackChunkName: "chunk-admin" */ './views/dashboard.vue');
const Users = () => import(/* webpackChunkName: "chunk-admin" */ './views/users.vue');

export default [
  {
    component: Dashboard,
    path: '/admin',
  },
  {
    component: Users,
    path: '/admin/users',
  },
];
