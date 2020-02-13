const Books = () => import(/* webpackChunkName: "chunk-books" */ './views/books.vue');

export default [
  {
    component: Books,
    path: '/books',
  },
];
