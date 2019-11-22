import Vue from 'vue';
import Router from 'vue-router';

import Index from '@/views/index.vue';
import About from '@/views/about.vue';
import Download from '@/views/download.vue';
import Terms from '@/views/terms.vue';
import Privacy from '@/views/privacy.vue';
import Security from '@/views/security.vue';
import Contact from '@/views/contact.vue';
import P404 from '@/views/404.vue';

import FeaturesRoutes from './features/routes';


Vue.use(Router);

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      component: Index,
    },
    {
      path: '/about',
      component: About,
    },
    {
      path: '/contact',
      component: Contact,
    },
    {
      path: '/download',
      component: Download,
    },
    {
      path: '/terms',
      component: Terms,
    },
    {
      path: '/privacy',
      component: Privacy,
    },
    {
      path: '/security',
      component: Security,
    },

    ...FeaturesRoutes,

    {
      component: P404,
      path: '**',
    },
  ],
});
