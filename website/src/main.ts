import Vue from 'vue';
import router from '@/views/routes';
import vuetify from '@/plugins/vuetify';
import App from '@/app.vue';
import { i18n } from '@/i18n';

// because GitLab pages does not support server side redirects
function redirect(domain: string) {
  const { hostname } = window.location;
  if (hostname !== domain && hostname !== 'localhost' && hostname !== '127.0.0.1') {
    window.location.href = `https://${domain}`;
  }
}
redirect('bloom.sh');

// init stage dependant stuff
if (process.env.ENV === 'development') {
  Vue.config.productionTip = true;
} else {
  Vue.config.productionTip = false;
}


new Vue({
  router,
  vuetify,
  i18n,
  render: (h) => h(App),
}).$mount('#app');
