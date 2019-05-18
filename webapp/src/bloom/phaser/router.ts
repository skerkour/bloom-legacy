const IndexView = () => import(/* webpackChunkName: "chunk-platform" */ './views/Index.vue'); // tslint:disable-line
const ProfilesView = () => import(/* webpackChunkName: "chunk-platform" */ './views/Profiles.vue'); // tslint:disable-line
const ReportView = () => import(/* webpackChunkName: "chunk-platform" */ './views/Report.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
      },
      layout: 'unauthenticated',
      service: 'platform',
    },
    path: '/platform/phaser',
  },
  {
    component: ReportView,
    meta: {
      auth: {
        layout: 'authenticated',
        redirect: '/platform/phaser',
        required: true,
      },
      service: 'platform',
    },
    path: '/platform/phaser/:scan_id/reports/:report_id',
  },
  {
    component: ProfilesView,
    meta: {
      auth: {
        layout: 'authenticated',
        redirect: '/platform/phaser',
        required: true,
      },
      service: 'platform',
    },
    path: '/platform/phaser/profiles',
  },
];
