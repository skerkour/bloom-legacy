import VueI18n from 'vue-i18n';
import Vue from 'vue';
import { getLang } from '@/utils/lang';

import en from './en';
import fr from './fr';

Vue.use(VueI18n);

const messages = {
  en,
  fr,
};

// Create VueI18n instance with options
export const i18n = new VueI18n({
  locale: getLang(), // set locale
  fallbackLocale: 'en',
  messages, // set locale messages
});
