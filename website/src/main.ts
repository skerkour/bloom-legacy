import Vue from 'vue';
import App from '@/app.vue';
import router from '@/router';
import vuetify from '@/plugins/vuetify';

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
  render: (h) => h(App),
}).$mount('#app');
