const IndexView = () => import(/* webpackChunkName: "chunk-gallery" */ './views/Index.vue'); // tslint:disable-line
const AlbumsView = () => import(/* webpackChunkName: "chunk-gallery" */ './views/albums/Index.vue'); // tslint:disable-line
const AlbumView = () => import(/* webpackChunkName: "chunk-gallery" */ './views/albums/Album.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      service: 'gallery',
    },
    path: '/gallery',
  },
  {
    component: AlbumsView,
    meta: {
      auth: {
        redirect: '/gallery',
        required: true,
      },
      service: 'gallery',
    },
    path: '/gallery/albums',
  },
  {
    component: AlbumView,
    meta: {
      auth: {
        redirect: '/gallery',
        required: true,
      },
      service: 'gallery',
    },
    path: '/gallery/albums/:album_id',
  },
];
