const Gallery = () => import(/* webpackChunkName: "chunk-gallery" */ './views/gallery.vue');
const Albums = () => import(/* webpackChunkName: "chunk-gallery" */ './views/albums.vue');
const Album = () => import(/* webpackChunkName: "chunk-gallery" */ './views/albums/album.vue');

export default [
  {
    component: Gallery,
    path: '/gallery',
  },
  {
    component: Albums,
    path: '/gallery/albums',
  },
  {
    component: Album,
    path: '/gallery/albums/:album_id',
  },
];
