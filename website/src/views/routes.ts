import Vue from 'vue';
import Router from 'vue-router';

import Index from '@/views/index.vue';
import Download from '@/views/download.vue';
import Contact from '@/views/contact.vue';
import P404 from '@/views/404.vue';
import Blog from '@/views/blog.vue';
import Pricing from '@/views/pricing.vue';

import FeaturesRoutes from './features/routes';

const Security = () => import(/* webpackChunkName: "chunk-organization" */ '@/views/security.vue');
const Privacy = () => import(/* webpackChunkName: "chunk-organization" */ '@/views/privacy.vue');
const Terms = () => import(/* webpackChunkName: "chunk-organization" */ '@/views/terms.vue');
const About = () => import(/* webpackChunkName: "chunk-organization" */ '@/views/about.vue');
const FAQ = () => import(/* webpackChunkName: "chunk-faq" */ '@/views/faq.vue');

Vue.use(Router);

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  scrollBehavior(to, from, savedPosition) {
    return { x: 0, y: 0 };
  },
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
      path: '/dl',
      redirect: '/download',
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
    {
      path: '/pricing',
      component: Pricing,
    },
    {
      path: '/faq',
      component: FAQ,
    },

    {
      path: '/blog**',
      component: Blog,
    },

    ...FeaturesRoutes,

    {
      component: P404,
      path: '**',
    },
  ],
});
