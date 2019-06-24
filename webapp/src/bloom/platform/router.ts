const IndexView = () => import(/* webpackChunkName: "chunk-platform" */ './views/Index.vue'); // tslint:disable-line
import PhaserRoutes from '../phaser/router';

export default [
  {
    component: IndexView,
    meta: {
      service: 'platform',
    },
    path: '/platform',
  },
  ...PhaserRoutes,
];
