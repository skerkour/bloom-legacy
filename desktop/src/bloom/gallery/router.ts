const Gallery = () => import(/* webpackChunkName: "chunk-gallery" */ './views/gallery.vue');
const Albums = () => import(/* webpackChunkName: "chunk-gallery" */ './views/albums.vue');

export default [
  {
    component: Gallery,
    path: '/gallery',
  },
  {
    component: Albums,
    path: '/gallery/albums',
  },
];
