const IndexView = () => import(/* webpackChunkName: "chunk-drive" */ './views/Index.vue'); // tslint:disable-line
const TrashView = () => import(/* webpackChunkName: "chunk-drive" */ './views/Trash.vue'); // tslint:disable-line

export default [
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
      },
      layout: 'unauthenticated',
      service: 'drive',
    },
    path: '/drive',
  },
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
        redirect: '/drive',
        required: true,
      },
      service: 'drive',
    },
    path: '/drive/folders/:folder_id',
  },
  {
    component: TrashView,
    meta: {
      auth: {
        layout: 'authenticated',
        redirect: '/drive',
        required: true,
      },
      service: 'drive',
    },
    path: '/drive/trash',
  },
];
