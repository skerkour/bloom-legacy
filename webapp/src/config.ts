class Config {
  ENV: string = '';
  SENTRY_URL: string = '';
  HOST: string = '';
  STRIPE_PUBLIC_KEY: string = '';
  API_BASE_URL: string = '';
  VERSION: string = '';

  constructor() {
    if ((window as any).__bloom) {
      const config = (window as any).__bloom.config;
      this.ENV = config.ENV;
      this.SENTRY_URL = config.SENTRY_URL;
      this.HOST = config.HOST;
      this.STRIPE_PUBLIC_KEY = config.STRIPE_PUBLIC_KEY;
      this.API_BASE_URL = config.API_BASE_URL;
      this.VERSION = config.VERSION;
    } else {
      // Check environement
      [
        'NODE_ENV',
        'VUE_APP_SENTRY_URL',
        'VUE_APP_HOST',
        'VUE_APP_STRIPE_PUBLIC_KEY',
      ].forEach((env_var) => {
        if (!env_var) {
          throw new Error(`Missing environment variable: ${env_var}`);
        }
      });
      this.ENV = process.env.NODE_ENV;
      this.SENTRY_URL = process.env.VUE_APP_SENTRY_URL;
      this.HOST = process.env.VUE_APP_HOST;
      this.STRIPE_PUBLIC_KEY = process.env.VUE_APP_STRIPE_PUBLIC_KEY;
      this.API_BASE_URL = process.env.VUE_APP_API_BASE_URL;
      this.VERSION = process.env.VERSION;
    }
  }
}

export default new Config();
