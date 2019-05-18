import Index from './views/Index.vue';

export default [
  {
    component: Index,
    meta: {
      auth: {
        layout: 'authenticated',
      },
      layout: 'unauthenticated',
      service: 'calendar',
    },
    path: '/calendar',
  },
];
