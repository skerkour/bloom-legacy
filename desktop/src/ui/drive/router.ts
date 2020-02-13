const Drive = () => import(/* webpackChunkName: "chunk-drive" */ './views/drive.vue');
const Trash = () => import(/* webpackChunkName: "chunk-drive" */ './views/trash.vue');

export default [
  {
    component: Drive,
    path: '/drive',
  },
  {
    component: Trash,
    path: '/drive/trash',
  },
];
