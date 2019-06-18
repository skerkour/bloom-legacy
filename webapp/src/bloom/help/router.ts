const IndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/Index.vue'); // tslint:disable-line
const ArcadeIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/arcade/Index.vue'); // tslint:disable-line
const BitflowIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/bitflow/Index.vue'); // tslint:disable-line
const CalendarIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/calendar/Index.vue'); // tslint:disable-line
const ContactsIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/contacts/Index.vue'); // tslint:disable-line
const DriveIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/drive/Index.vue'); // tslint:disable-line
const GalleryIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/gallery/Index.vue'); // tslint:disable-line
const MusicIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/music/Index.vue'); // tslint:disable-line
const MyAccountIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/myaccount/Index.vue'); // tslint:disable-line
const NotesIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/notes/Index.vue'); // tslint:disable-line
const PhaserIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/phaser/Index.vue'); // tslint:disable-line
const PlatformIndexView = () => import(/* webpackChunkName: "chunk-help" */ './views/platform/Index.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help',
  },
  {
    component: ArcadeIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/arcade',
  },
  {
    component: BitflowIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/bitflow',
  },
  {
    component: CalendarIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/calendar',
  },
  {
    component: ContactsIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/contacts',
  },
  {
    component: DriveIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/drive',
  },
  {
    component: GalleryIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/gallery',
  },
  {
    component: MusicIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/music',
  },
  {
    component: MyAccountIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/myaccount',
  },
  {
    component: NotesIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/notes',
  },
  {
    component: PhaserIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/phaser',
  },
  {
    component: PlatformIndexView,
    meta: {
      auth: { layout: 'authenticated' },
      layout: 'unauthenticated',
      service: 'help',
    },
    path: '/help/platform',
  },
];
