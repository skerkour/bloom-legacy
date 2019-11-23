import VueI18n from 'vue-i18n';
import Vue from 'vue';

import en from './en';
import fr from './fr';
import es from './es';

Vue.use(VueI18n);

const messages = {
  en,
  es,
  fr,
};

// Create VueI18n instance with options
export default new VueI18n({
  locale: 'fr', // set locale
  messages, // set locale messages
});
