class Config {
  NODE_ENV: string = '';
  VUE_APP_SENTRY_URL: string = '';
  VUE_APP_HOST: string = '';
  VUE_APP_STRIPE_PUBLIC_KEY: string = '';
  VUE_APP_API_BASE_URL: string = '';

  constructor() {
    if ((window as any).__bloom_env) {
      const env = (window as any).__bloom_env;
      this.NODE_ENV = env.NODE_ENV;
      this.VUE_APP_SENTRY_URL = env.VUE_APP_SENTRY_URL;
      this.VUE_APP_HOST = env.VUE_APP_HOST;
      this.VUE_APP_STRIPE_PUBLIC_KEY = env.VUE_APP_STRIPE_PUBLIC_KEY;
      this.VUE_APP_API_BASE_URL = env.VUE_APP_API_BASE_URL;
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
      this.NODE_ENV = process.env.NODE_ENV;
      this.VUE_APP_SENTRY_URL = process.env.VUE_APP_SENTRY_URL;
      this.VUE_APP_HOST = process.env.VUE_APP_HOST;
      this.VUE_APP_STRIPE_PUBLIC_KEY = process.env.VUE_APP_STRIPE_PUBLIC_KEY;
      this.VUE_APP_API_BASE_URL = process.env.VUE_APP_API_BASE_URL;
    }
  }
}

export default new Config();
