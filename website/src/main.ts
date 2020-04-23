import Vue from 'vue';
import router from '@/views/routes';
import vuetify from '@/plugins/vuetify';
import App from '@/app.vue';
import { i18n } from '@/i18n';

// init stage dependant stuff
if (process.env.ENV === 'development') {
  Vue.config.productionTip = true;
} else {
  Vue.config.productionTip = false;
}


const vue = new Vue({
  router,
  vuetify,
  i18n,
  render: (h) => h(App),
}).$mount('#app');

if (vue.$route.query.lang) {
  vue.$i18n.locale = vue.$route.query.lang! as string;
}
