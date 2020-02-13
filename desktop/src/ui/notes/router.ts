const Notes = () => import(/* webpackChunkName: "chunk-notes" */ './views/Notes.vue');
const Archive = () => import(/* webpackChunkName: "chunk-notes" */ './views/Archive.vue');

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
