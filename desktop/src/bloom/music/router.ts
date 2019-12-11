const Songs = () => import(/* webpackChunkName: "chunk-music" */ './views/songs.vue');
const Playlists = () => import(/* webpackChunkName: "chunk-music" */ './views/playlists.vue');
const Playlist = () => import(/* webpackChunkName: "chunk-music" */ './views/playlists/playlist.vue');

export default [
  {
    component: Songs,
    path: '/music',
  },
  {
    component: Playlists,
    path: '/music/playlists',
  },
  {
    component: Playlist,
    path: '/music/playlists/:playlist_id',
  },
];
