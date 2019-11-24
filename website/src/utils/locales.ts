import store from 'store';

export function detectLocale(): string {
  let locale = store.get('locale');
  if (locale) {
    return locale;
  }

  locale = navigator.language || (navigator as any).userLanguage;

  locale = (!locale || locale.length < 2) ? 'en' : locale.substring(0, 2);
  store.set('locale', locale);
  return locale;
}

export function saveLocale(locale: string) {
  store.set('locale', locale);
}
