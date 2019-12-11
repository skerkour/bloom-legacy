const Songs = () => import(/* webpackChunkName: "chunk-music" */ './views/songs.vue');
const Playlists = () => import(/* webpackChunkName: "chunk-music" */ './views/playlists.vue');

export default [
  {
    component: Songs,
    path: '/music',
  },
  {
    component: Playlists,
    path: '/music/playlists',
  },
];
