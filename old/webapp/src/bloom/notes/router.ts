const IndexView = () => import(/* webpackChunkName: "chunk-notes" */ './views/Index.vue'); // tslint:disable-line
const ArchiveView = () => import(/* webpackChunkName: "chunk-notes" */ './views/Archive.vue'); // tslint:disable-line
const TrashView = () => import(/* webpackChunkName: "chunk-notes" */ './views/Trash.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      service: 'notes',
    },
    path: '/notes',
  },
  {
    component: ArchiveView,
    meta: {
      auth: {
        redirect: '/notes',
        required: true,
      },
      service: 'notes',
    },
    path: '/notes/archive',
  },
  {
    component: TrashView,
    meta: {
      auth: {
        redirect: '/notes',
        required: true,
      },
      service: 'notes',
    },
    path: '/notes/trash',
  },
];
