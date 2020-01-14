import store from 'store';

const STORE_KEY = 'lang';

export function saveLang(locale: string) {
  store.set(STORE_KEY, locale);
  document.querySelector('html')!.setAttribute('lang', locale);
}

export function getLang(): string {
  let locale = store.get(STORE_KEY);
  if (locale) {
    return locale;
  }

  locale = navigator.language || (navigator as any).userLanguage;

  locale = (!locale || locale.length < 2) ? 'en' : locale.substring(0, 2);
  saveLang(locale);
  return locale;
}
