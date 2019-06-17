class Config {
  ENV: string = '';
  SENTRY_URL: string = '';
  HOST: string = '';
  STRIPE_PUBLIC_KEY: string = '';
  API_BASE_URL: string = '';

  constructor() {
    if ((window as any).__bloom_env) {
      const env = (window as any).__bloom_env;
      this.ENV = env.ENV;
      this.SENTRY_URL = env.SENTRY_URL;
      this.HOST = env.HOST;
      this.STRIPE_PUBLIC_KEY = env.STRIPE_PUBLIC_KEY;
      this.API_BASE_URL = env.API_BASE_URL;
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
    }
  }
}

export default new Config();
