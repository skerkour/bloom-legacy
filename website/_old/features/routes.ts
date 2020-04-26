import Index from './index.vue';

const Arcade = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/arcade.vue');
const Bitflow = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/bitflow.vue');
// const Books =() => import(/* webpackChunkName: "chunk-features" */ '@/views/features/books.vue');
const Calendar = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/calendar.vue');
// const Chat = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/chat.vue');
const Contacts = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/contacts.vue');
const Files = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/files.vue');
const Notes = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/notes.vue');

export default [
  {
    component: Index,
    path: '/features',
  },
  {
    component: Arcade,
    path: '/features/arcade',
  },
  {
    component: Bitflow,
    path: '/features/bitflow',
  },
  {
    component: Calendar,
    path: '/features/calendar',
  },
  {
    component: Contacts,
    path: '/features/contacts',
  },
  {
    component: Files,
    path: '/features/files',
  },
  {
    component: Notes,
    path: '/features/notes',
  },
];
