const Files = () => import(/* webpackChunkName: "chunk-files" */ './views/files.vue');
const Trash = () => import(/* webpackChunkName: "chunk-files" */ './views/trash.vue');

export default [
  {
    component: Files,
    path: '/files',
  },
  {
    component: Trash,
    path: '/files/trash',
  },
];
