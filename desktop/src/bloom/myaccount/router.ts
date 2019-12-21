const Profile = () => import(/* webpackChunkName: "chunk-myaccount" */ './views/profile.vue');
const Billing = () => import(/* webpackChunkName: "chunk-myaccount" */ './views/billing.vue');
const Security = () => import(/* webpackChunkName: "chunk-myaccount" */ './views/security.vue');
const Devices = () => import(/* webpackChunkName: "chunk-myaccount" */ './views/devices.vue');

export default [
  {
    component: Profile,
    path: '/myaccount',
  },
  {
    component: Billing,
    path: '/myaccount/billing',
  },
  {
    component: Security,
    path: '/myaccount/security',
  },
  {
    component: Devices,
    path: '/myaccount/devices',
  },
];
