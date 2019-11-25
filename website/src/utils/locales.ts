import store from 'store';


export function saveLocale(locale: string) {
  store.set('locale', locale);
  document.querySelector('html')!.setAttribute('lang', locale);
}

export function getLocale(): string {
  let locale = store.get('locale');
  if (locale) {
    return locale;
  }

  locale = navigator.language || (navigator as any).userLanguage;

  locale = (!locale || locale.length < 2) ? 'en' : locale.substring(0, 2);
  saveLocale(locale);
  return locale;
}
