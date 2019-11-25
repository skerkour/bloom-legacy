import VueI18n from 'vue-i18n';
import Vue from 'vue';
import { getLocale } from '@/utils/locales';

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
export const i18n = new VueI18n({
  locale: getLocale(), // set locale
  fallbackLocale: 'en',
  messages, // set locale messages
});
