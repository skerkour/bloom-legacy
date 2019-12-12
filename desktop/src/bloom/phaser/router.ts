const Scans = () => import(/* webpackChunkName: "chunk-phaser" */ './views/scans.vue');
const Report = () => import(/* webpackChunkName: "chunk-phaser" */ './views/scans/report.vue');

export default [
  {
    component: Scans,
    path: '/phaser',
  },
  {
    component: Report,
    path: '/phaser/scans/:scan_id/reports',
  },
  {
    component: Report,
    path: '/phaser/scans/:scan_id/reports/:report_id',
  },
];
