const IndexView = () => import(/* webpackChunkName: "chunk-myaccount" */ './views/Index.vue'); // tslint:disable-line
const SecurityView = () => import(/* webpackChunkName: "chunk-myaccount" */'./views/Security.vue'); // tslint:disable-line
const DevicesView = () => import(/* webpackChunkName: "chunk-myaccount" */'./views/Devices.vue'); // tslint:disable-line
const AuditView = () => import(/* webpackChunkName: "chunk-myaccount" */'./views/Audit.vue'); // tslint:disable-line
// const BillingView = () => import(/* webpackChunkName: "chunk-myaccount" */'./views/Billing.vue');
import ComingSoon from '../kernel/components/ComingSoon.vue';
import AccountRecoveryRequestForm from './components/RequestRecoveryForm.vue';
import RecoveryForm from './components/RecoveryForm.vue';

export default [
  // account recovery
  {
    component: RecoveryForm,
    meta: {
      service: 'myaccount',
    },
    name: 'myaccount/recovery',
    path: '/myaccount/recovery',
  },
  {
    component: AccountRecoveryRequestForm,
    meta: {
      service: 'myaccount',
    },
    name: 'myaccount/request_recovery',
    path: '/myaccount/recovery/request',
  },
  {
    component: IndexView,
    meta: {
      auth: {
        required: true,
      },
      service: 'myaccount',
    },
    name: 'myaccount/index',
    path: '/myaccount',
  },
  {
    component: ComingSoon,
    meta: {
      auth: {
        required: true,
      },
      service: 'myaccount',
    },
    name: 'myaccount/billing',
    path: '/myaccount/billing',
  },
  {
    component: SecurityView,
    meta: {
      auth: {
        required: true,
      },
      service: 'myaccount',
    },
    name: 'myaccount/security',
    path: '/myaccount/security',
  },
  {
    component: DevicesView,
    meta: {
      auth: {
        required: true,
      },
      service: 'myaccount',
    },
    name: 'myaccount/devices',
    path: '/myaccount/devices',
  },
  {
    component: AuditView,
    meta: {
      auth: {
        required: true,
      },
      service: 'myaccount',
    },
    name: 'myaccount/audit',
    path: '/myaccount/audit',
  },
];
