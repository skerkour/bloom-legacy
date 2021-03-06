import Vue from 'vue';
import Router from 'vue-router';

import AuthRouter from '@/ui/auth/router';
import PreferencesRouter from '@/ui/preferences/router';
import NotesRouter from '@/ui/notes/router';
import ArcadeRouter from '@/ui/arcade/router';
import CalendarRouter from '@/ui/calendar/router';
import FilesRouter from '@/ui/files/router';
import ContactsRouter from '@/ui/contacts/router';
import CalculatorRouter from '@/ui/calculator/router';
import AdminRouter from '@/ui/admin/router';
import MyAccountRouter from '@/ui/myaccount/router';
import GroupsRouter from '@/ui/groups/router';
import KernelRouter from '@/ui/kernel/router';


import store from '@/store';

Vue.use(Router);

const router = new Router({
  mode: 'history',
  // base: process.env.BASE_URL,
  routes: [
    ...AuthRouter,
    ...PreferencesRouter,
    ...NotesRouter,
    ...CalendarRouter,
    ...ArcadeRouter,
    ...FilesRouter,
    ...ContactsRouter,
    ...CalculatorRouter,
    ...AdminRouter,
    ...MyAccountRouter,
    ...GroupsRouter,
    ...KernelRouter,
    {
      path: '**',
      redirect: '/notes',
    },
  ],
});


router.beforeEach((to, _, next) => {
  if (store.state.me === null) {
    if (to.path.startsWith('/auth')) {
      next();
    } else {
      next({ path: '/auth/registration/start' });
    }
  } else {
    next();
  }

  // if (to.meta.auth && to.meta.auth.required === true) {
  //   if (store.state.isAuthenticated) {
  //     next();
  //   } else if (to.meta.auth.redirect) {
  //     next({ path: to.meta.auth.redirect });
  //   } else {
  //     next({ path: '/sign-in' });
  //   }
  // } else if (to.meta.auth && to.meta.auth.forbidden) {
  //   if (store.state.isAuthenticated) {
  //     next({ path: '/' });
  //   } else {
  //     next();
  //   }
  // } else {
  //   next();
  // }
});

export default router;
