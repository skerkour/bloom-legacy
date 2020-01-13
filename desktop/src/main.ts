import Vue from 'vue';
import { ipcRenderer } from 'electron';
import App from '@/App.vue';
import router from '@/router';
import store from '@/store';
import vuetify from '@/plugins/vuetify';
import filters from '@/filters';
import core from './core';

const { log, Level } = require('@bloom42/astro');


if (process.env.NODE_ENV === 'development') {
  Vue.config.productionTip = true;
} else {
  Vue.config.productionTip = false;
  if (process.env.NODE_ENV === 'production') {
    log.config({ level: Level.INFO });
  }
}


window.onunload = async () => {
  await ipcRenderer.send('server:stop');
  window.location.reload();
};

async function main() {
  await ipcRenderer.send('server:start');
  await core.init();

  Vue.use(filters);

  new Vue({
    router,
    store,
    vuetify,
    render: (h) => h(App),
  }).$mount('#app');
}

main();
