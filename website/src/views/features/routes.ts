const Index = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/index.vue');
const Notes = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/notes.vue');

export default [
  {
    component: Index,
    path: '/features',
  },
  {
    component: Notes,
    path: '/features/notes',
  },
];
