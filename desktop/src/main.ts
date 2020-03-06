import Vue from 'vue';
import { ipcRenderer } from 'electron';
import App from '@/App.vue';
import router from '@/router';
import store, { Mutations } from '@/store';
import vuetify from '@/plugins/vuetify';
import filters from '@/filters';
import core from '@/core';
import { log, Level } from '@/libs/rz';


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

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function main() {
  await ipcRenderer.send('server:start');
  await sleep(1000);
  const signedIn = await core.init();
  if (signedIn !== null) {
    store.commit(Mutations.SIGN_IN.toString(), signedIn);
  }

  Vue.use(filters);

  new Vue({
    router,
    store,
    vuetify,
    render: (h) => h(App),
  }).$mount('#app');
}

main();
