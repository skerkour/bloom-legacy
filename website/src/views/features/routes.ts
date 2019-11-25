import Index from './index.vue';

const Arcade = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/arcade.vue');
const Bitflow = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/bitflow.vue');
const Books = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/books.vue');
const Calendar = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/calendar.vue');
const Chat = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/chat.vue');
const Contacts = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/contacts.vue');
const Drive = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/drive.vue');
const Gallery = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/gallery.vue');
const Music = () => import(/* webpackChunkName: "chunk-features" */ '@/views/features/music.vue');
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
    component: Books,
    path: '/features/books',
  },
  {
    component: Calendar,
    path: '/features/calendar',
  },
  {
    component: Chat,
    path: '/features/chat',
  },
  {
    component: Contacts,
    path: '/features/contacts',
  },
  {
    component: Drive,
    path: '/features/drive',
  },
  {
    component: Gallery,
    path: '/features/gallery',
  },
  {
    component: Music,
    path: '/features/music',
  },
  {
    component: Notes,
    path: '/features/notes',
  },
];
