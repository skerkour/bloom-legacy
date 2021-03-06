module.exports = {
  chainWebpack: (config) => {
    config.plugin('define').tap((args) => {
      const v = JSON.stringify(require('./package.json').version); // eslint-disable-line global-require
      args[0]['process.env'].VERSION = v; // eslint-disable-line no-param-reassign
      return args;
    });

    config.module
      .rule('vue')
      .use('vue-loader')
      .loader('vue-loader')
      .tap((options) => {
        options.transformAssetUrls = { // eslint-disable-line
          video: ['src', 'poster'],
          source: 'src',
          img: 'src',
          image: ['xlink:href', 'href'],
          use: ['xlink:href', 'href'],
          'v-img': 'src',
        };

        return options;
      });
  },
  pwa: {
    name: 'Bloom',
    iconPaths: {
      favicon64: 'kernel/imgs/icons/bloom_64.png',
      favicon32: 'kernel/imgs/icons/bloom_64.png',
      favicon16: 'kernel/imgs/icons/bloom_32.png',
      appleTouchIcon: 'kernel/imgs/icons/bloom_256.png',
      maskIcon: 'kernel/imgs/icons/bloom_256.png',
      msTileImage: 'kernel/imgs/icons/bloom_256.png',
    },
  },
  pluginOptions: {
    electronBuilder: {
      chainWebpackRendererProcess: (config) => {
        if (process.env.NODE_ENV === 'development') {
          config.plugins.delete('prefetch');
        }
      },
      builderOptions: {
        productName: 'Bloom',
        appId: 'com.bloom42.bloom',
        extraResources: ['bloomd'],
        publish: [
          {
            provider: 'generic',
            url: 'https://bloom42-updates.s3.fr-par.scw.cloud/desktop',
          },
        ],
        mac: {
          category: 'public.app-category.productivity', // see https://developer.apple.com/library/archive/documentation/General/Reference/InfoPlistKeyReference/Articles/LaunchServicesKeys.html#//apple_ref/doc/uid/TP40009250-SW8
          darkModeSupport: true,
          hardenedRuntime: true,
          icon: 'build/icons/mac/icon.icns',
          target: 'dmg',
        },
        linux: {
          category: 'Utility', // https://specifications.freedesktop.org/menu-spec/latest/apa.html
          icon: 'build/icons/png',
        },
      },
    },
  },
};
