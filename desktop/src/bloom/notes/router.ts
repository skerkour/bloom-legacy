const Notes = () => import(/* webpackChunkName: "chunk-notes" */ './views/Notes.vue');

export default [
  {
    component: Notes,
    path: '/notes',
  },
];
