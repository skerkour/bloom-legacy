import Vue from 'vue';
import Router from 'vue-router';
import Index from '@/views/index.vue'; // eslint-disable-line
const About = () => import(/* webpackChunkName: "about" */ '@/views/about.vue');
import Download from '@/views/download.vue'; // eslint-disable-line
import FeaturesIndex from '@/views/features/index.vue'; // eslint-disable-line


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
      path: '/download',
      component: Download,
    },
    {
      path: '/features',
      component: FeaturesIndex,
    },
  ],
});
