/* tslint:disable */
import Vue from 'vue';
import * as Sentry from '@sentry/browser';
import * as Integrations from '@sentry/integrations';
import Vuetify from 'vuetify';
const { log, Level } = require('@bloom42/astro');

import 'vuetify/dist/vuetify.min.css';
import '@mdi/font/css/materialdesignicons.css';

import App from '@/App.vue';
import config from './config';
import router from '@/bloom/kernel/router';
import store from '@/store';
import api from '@/bloom/kernel/api';
import filters from '@/bloom/kernel/filters';

import VuetifyToast from '@/bloom/kernel/components/Toast';
Vue.use(VuetifyToast);
declare module 'vue/types/vue' {
  interface Vue {
    $toast: any;
  }
}


// kernel
import Toolbar from '@/bloom/kernel/components/Toolbar.vue';
import Footer from '@/bloom/kernel/components/Footer.vue';

Vue.component('blm-toolbar', Toolbar);
Vue.component('blm-footer', Footer);

// import './registerServiceWorker';

// init sentry for bug tracking
Sentry.init({
  dsn: config.SENTRY_URL,
  environment: config.ENV,
  integrations: [new Integrations.Vue({ Vue })],
});

// init stage dependant stuff
if (config.ENV === 'development') {
  Vue.config.productionTip = true;
} else {
  Vue.config.productionTip = false;

  if (config.ENV === 'production') {
    log.config({ level: Level.INFO });
  }
}

log.with({ config }).debug('config loaded');

// init libraries and components
Vue.use(Vuetify, {
  iconfont: 'mdi',
  icons: {
    loading: 'mdi-loading',
  },
});


Vue.use(filters);
api.init();

new Vue({
  render: (h: any) => h(App),
  router,
  store,
}).$mount('#app');
