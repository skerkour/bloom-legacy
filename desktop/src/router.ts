import Vue from 'vue';
import Router from 'vue-router';

import AuthRouter from '@/bloom/auth/router';
import KernelRouter from '@/bloom/kernel/router';
import ChatRouter from '@/bloom/chat/router';
import PreferencesRouter from '@/bloom/preferences/router';
import MusicRouter from '@/bloom/music/router';
import NotesRouter from '@/bloom/notes/router';
import ArcadeRouter from '@/bloom/arcade/router';
import CalendarRouter from '@/bloom/calendar/router';
import DriveRouter from '@/bloom/drive/router';
import BitflowRouter from '@/bloom/bitflow/router';
import ContactsRouter from '@/bloom/contacts/router';
import GalleryRouter from '@/bloom/gallery/router';
import CalculatorRouter from '@/bloom/calculator/router';
import PhaserRouter from '@/bloom/phaser/router';
import AdminRouter from '@/bloom/admin/router';
import MyAccountRouter from '@/bloom/myaccount/router';
import BooksRouter from '@/bloom/books/router';

import store from '@/store';

Vue.use(Router);

const router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    ...AuthRouter,
    ...ChatRouter,
    ...KernelRouter,
    ...PreferencesRouter,
    ...MusicRouter,
    ...NotesRouter,
    ...CalendarRouter,
    ...ArcadeRouter,
    ...DriveRouter,
    ...BitflowRouter,
    ...ContactsRouter,
    ...GalleryRouter,
    ...CalculatorRouter,
    ...PhaserRouter,
    ...AdminRouter,
    ...MyAccountRouter,
    ...BooksRouter,
    {
      path: '**',
      redirect: '/chat',
    },
  ],
});


router.beforeEach((to, _, next) => {
  if (store.state.is_authenticated === false) {
    if (to.path.startsWith('/auth')) {
      next();
    } else {
      next({ path: '/auth/registration/start' });
    }
  } else {
    next();
  }

  // if (to.meta.auth && to.meta.auth.required === true) {
  //   if (store.state.is_authenticated) {
  //     next();
  //   } else if (to.meta.auth.redirect) {
  //     next({ path: to.meta.auth.redirect });
  //   } else {
  //     next({ path: '/sign-in' });
  //   }
  // } else if (to.meta.auth && to.meta.auth.forbidden) {
  //   if (store.state.is_authenticated) {
  //     next({ path: '/' });
  //   } else {
  //     next();
  //   }
  // } else {
  //   next();
  // }
});

export default router;
