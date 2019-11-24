import VueI18n from 'vue-i18n';
import Vue from 'vue';
import { detectLocale } from '@/utils/locales';

import en from './en';
import fr from './fr';
import es from './es';
import pt from './pt';

Vue.use(VueI18n);

const messages = {
  en,
  es,
  fr,
  pt,
};

// Create VueI18n instance with options
export default new VueI18n({
  locale: detectLocale(), // set locale
  fallbackLocale: 'en',
  messages, // set locale messages
});
