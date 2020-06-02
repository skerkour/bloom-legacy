const Notes = () => import(/* webpackChunkName: "chunk-notes" */ './views/notes.vue');
const Archive = () => import(/* webpackChunkName: "chunk-notes" */ './views/archive.vue');

export default [
  {
    component: Notes,
    path: '/notes',
  },
  {
    component: Archive,
    path: '/notes/archive',
  },
];
