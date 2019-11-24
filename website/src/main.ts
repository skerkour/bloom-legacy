import Vue from 'vue';
import router from '@/views/routes';
import vuetify from '@/plugins/vuetify';
import App from '@/app.vue';
import { i18n } from '@/i18n';

const { log, Level } = require('@bloom42/astro');

// init stage dependant stuff
if (process.env.ENV === 'development') {
  Vue.config.productionTip = true;
} else {
  Vue.config.productionTip = false;

  if (process.env.ENV === 'production') {
    log.config({ level: Level.INFO });
  }
}


new Vue({
  router,
  vuetify,
  i18n,
  render: (h) => h(App),
}).$mount('#app');
