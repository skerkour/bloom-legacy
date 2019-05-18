const IndexView = () => import(/* webpackChunkName: "chunk-music" */ './views/Index.vue'); // tslint:disable-line
const PlaylistsView = () => import(/* webpackChunkName: "chunk-music" */ './views/playlists/Index.vue'); // tslint:disable-line
const PlaylistView = () => import(/* webpackChunkName: "chunk-music" */ './views/playlists/Playlist.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
      },
      layout: 'unauthenticated',
      service: 'music',
    },
    path: '/music',
  },
  {
    component: PlaylistsView,
    meta: {
      auth: {
        layout: 'authenticated',
        redirect: '/music',
        required: true,
      },
      service: 'music',
    },
    path: '/music/playlists',
  },
  {
    component: PlaylistView,
    meta: {
      auth: {
        layout: 'authenticated',
        redirect: '/music',
        required: true,
      },
      service: 'music',
    },
    path: '/music/playlists/:playlist_id',
  },
];
