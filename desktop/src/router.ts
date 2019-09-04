import Vue from 'vue';
import Router from 'vue-router';

import AuthRouter from '@/bloom/auth/router';

import Home from '@/views/Home.vue';
import store from '@/store';

Vue.use(Router);

const router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import(/* webpackChunkName: "about" */ './views/About.vue'),
    },

    ...AuthRouter,
    {
      redirect: '/',
      path: '**',
    },
  ],
});


router.beforeEach((to, _, next) => {
  if (store.state.is_authenticated === false) {
    if (to.path === '/sign-in' || to.path === '/register') {
      next();
    } else {
      next({ path: '/sign-in' });
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
