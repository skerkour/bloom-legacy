import Vue from 'vue';
import App from '@/App.vue';
import router from '@/router';
import store, { Mutations } from '@/store';
import vuetify from '@/plugins/vuetify';
import filters from '@/filters';
import core, { InitRes } from '@/core';
import { log, Level } from '@/libs/rz';

const { ipcRenderer } = window as any;


if (process.env.NODE_ENV === 'production') {
  Vue.config.productionTip = false;
  log.config({ level: Level.INFO });
}


window.onunload = async () => {
  await ipcRenderer.send('server:stop');
  window.location.reload();
};

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// little hack to remove in the future. see https://github.com/vuetifyjs/vuetify/issues/9999
const ignoreWarnMessage = 'The .native modifier for v-on is only valid on components but it was used on <div>.';
Vue.config.warnHandler = (msg: any, vm: any, trace: any) => { //eslint-disable-line
  // `trace` is the component hierarchy trace
  if (msg === ignoreWarnMessage) {
    msg = null; //eslint-disable-line
    vm = null; //eslint-disable-line
    trace = null; //eslint-disable-line
  }
};

async function main() {
  let res: InitRes | null = null;
  await ipcRenderer.send('server:start');

  while (true) { // eslint-disable-line
    try {
      res = await core.init(['theme']); // eslint-disable-line
      break;
    } catch (err) {
      await sleep(50); // eslint-disable-line
      continue; // eslint-disable-line
    }
  }

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
