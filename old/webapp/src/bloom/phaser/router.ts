const IndexView = () => import(/* webpackChunkName: "chunk-platform" */ './views/Index.vue'); // tslint:disable-line
const ProfilesView = () => import(/* webpackChunkName: "chunk-platform" */ './views/Profiles.vue'); // tslint:disable-line
const ReportView = () => import(/* webpackChunkName: "chunk-platform" */ './views/Report.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      service: 'platform',
    },
    path: '/platform/phaser',
  },
  {
    component: ReportView,
    meta: {
      auth: {
        redirect: '/platform/phaser',
        required: true,
      },
      service: 'platform',
    },
    path: '/platform/phaser/scans/:scan_id/reports/:report_id',
  },
  {
    component: ReportView,
    meta: {
      auth: {
        redirect: '/platform/phaser',
        required: true,
      },
      service: 'platform',
    },
    path: '/platform/phaser/scans/:scan_id/reports',
  },
  {
    component: ProfilesView,
    meta: {
      auth: {
        redirect: '/platform/phaser',
        required: true,
      },
      service: 'platform',
    },
    path: '/platform/phaser/profiles',
  },
];
