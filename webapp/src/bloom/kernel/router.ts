import Vue from 'vue';
import Router from 'vue-router';

import api from './api';

import Index from './views/Index.vue';
import About from './views/about/Index.vue';
const Contribute = () => import(/* webpackChunkName: "chunk-organization" */ './views/contribute/Index.vue'); // tslint:disable-line
const BecomeASponsor = () => import(/* webpackChunkName: "chunk-organization" */ './views/become-a-sponsor/Index.vue'); // tslint:disable-line
const PrivacyPage = () => import(/* webpackChunkName: "chunk-organization" */ './views/privacy/Index.vue'); // tslint:disable-line
import AppsPage from './views/apps/Index.vue';
const TermsPage = () => import(/* webpackChunkName: "chunk-organization" */ './views/terms/Index.vue'); // tslint:disable-line
import Contact from './views/contact/Index.vue';
import Security from './views/security/Index.vue';
import SignInForm from './components/SignInForm.vue';
import RegisterForm from './components/RegisterForm.vue';
import DefaultLayout from '@/bloom/kernel/layouts/Default.vue';
import VerifyAccount from './components/Verify.vue';
import SetupUsernameForm from './components/SetupUsernameForm.vue';
import P404 from './views/404.vue';
const OpenSourcePage = () => import(/* webpackChunkName: "chunk-organization" */ './views/open-source/Index.vue'); // tslint:disable-line
const LicensingPage = () => import(/* webpackChunkName: "chunk-organization" */ './views/licensing/Index.vue'); // tslint:disable-line


import MyAccountRoutes from '@/bloom/myaccount/router';
import DriveRoutes from '@/bloom/drive/router';
import PlatformRoutes from '@/bloom/platform/router';
import BitflowRoutes from '@/bloom/bitflow/router';
import NotesRoutes from '@/bloom/notes/router';
import ContactsRoutes from '@/bloom/contacts/router';
import CalendarRoutes from '@/bloom/calendar/router';
import ArcadeRoutes from '@/bloom/arcade/router';
import GalleryRoutes from '@/bloom/gallery/router';
import MusicRoutes from '@/bloom/music/router';
import AdminRoutes from '@/bloom/admin/router';
import HelpRoutes from '@/bloom/help/router';


Vue.use(Router);

const router = new Router({
  mode: 'history',
  scrollBehavior() {
    return { x: 0, y: 0 };
  },
  routes: [
    {
      component: Index,
      path: '/',
    },
    {
      component: About,
      path: '/about',
    },
    {
      component: Contact,
      path: '/contact',
    },
    {
      component: Security,
      path: '/security',
    },
    {
      component: TermsPage,
      path: '/terms',
    },
    {
      component: PrivacyPage,
      path: '/privacy',
    },
    {
      component: AppsPage,
      path: '/download-apps',
    },
    {
      component: Contribute,
      path: '/contribute',
    },
    {
      component: OpenSourcePage,
      path: '/open-source',
    },
    { path: '/opensource', redirect: '/open-source' },
    {
      component: LicensingPage,
      path: '/licensing',
    },
    {
      component: BecomeASponsor,
      path: '/become-a-sponsor',
    },
    {
      component: SignInForm,
      meta: {
        auth: {
          forbidden: true,
        },
        layout: 'auth',
      },
      path: '/sign-in',
    },
    { path: '/login', redirect: '/sign-in' },
    { path: '/signin', redirect: '/sign-in' },
    { path: '/sign_in', redirect: '/sign-in' },

    // registration
    {
      component: RegisterForm,
      meta: {
        auth: {
          forbidden: true,
        },
        layout: 'auth',
      },
      path: '/register',
    },
    { path: '/signup', redirect: '/register' },
    { path: '/sign-up', redirect: '/register' },
    { path: '/sign_up', redirect: '/register' },

    // onboarding
    {
      component: VerifyAccount,
      meta: {
        auth: {
          forbidden: true,
        },
        onboarding: true,
      },
      path: '/welcome/verify',
    },
    {
      component: SetupUsernameForm,
      meta: {
        auth: {
          forbidden: true,
        },
        onboarding: true,
      },
      path: '/welcome/username',
    },
    ...MyAccountRoutes,
    ...DriveRoutes,
    ...PlatformRoutes,
    ...BitflowRoutes,
    ...NotesRoutes,
    ...ContactsRoutes,
    ...CalendarRoutes,
    ...ArcadeRoutes,
    ...GalleryRoutes,
    ...MusicRoutes,
    ...AdminRoutes,
    ...HelpRoutes,
    {
      component: P404,
      path: '**',
    },
  ],
});

router.beforeEach((to, _, next) => {
  if (to.meta.auth && to.meta.auth.required === true) {
    if (api.is_authenticated() === true) {
      next();
    } else if (to.meta.auth.redirect) {
      next({ path: to.meta.auth.redirect });
    } else {
      next({ path: '/sign-in' });
    }
  } else if (to.meta.auth && to.meta.auth.forbidden) {
    if (api.is_authenticated()) {
      next({ path: '/' });
    } else {
      next();
    }
  } else {
    next();
  }

  // if (to.meta.auth === true) { // auth required
  //   if (api.is_authenticated() !== true) {
  //     if (to.meta.unauthenticated) {
  //       to.meta.layout = 'unauthenticated';
  //       next({ path: to.meta.unauthenticated });
  //     } else {
  //       next({ path: '/sign-in' });
  //     }
  //   } else {
  //     next();
  //   }
  // } else if (to.meta.auth === false) { // auth forbidden
  //   if (api.is_authenticated()) {
  //     next({ path: '/' });
  //   } else {
  //     next();
  //   }
  // } else {
  //   next();
  // }
});

export default router;
