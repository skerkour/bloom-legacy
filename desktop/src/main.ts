import Vue from 'vue';
import App from '@/App.vue';
import router from '@/router';
import store, { Mutations } from '@/store';
import vuetify from '@/plugins/vuetify';
import filters from '@/filters';
import core from '@/core';
import { log, Level } from '@/libs/rz';

const { ipcRenderer } = window as any;

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
  const res = await core.init(['theme']);
  if (res.preferences.me) {
    const params = {
      me: res.preferences.me,
      session: res.preferences.session,
    };
    store.commit(Mutations.SIGN_IN.toString(), params);
  }
  if (res.preferences.theme === 'dark') {
    store.commit(Mutations.SWITCH_DARK_MODE.toString());
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
