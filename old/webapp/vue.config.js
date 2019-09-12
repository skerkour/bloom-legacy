module.exports = {
  chainWebpack: config => {
    config.plugin('define').tap(args => {
      let v = JSON.stringify(require('./package.json').version)
      args[0]['process.env']['VERSION'] = v
      return args;
    })
  },
  pwa: {
    name: 'Bloom',
    iconPaths: {
      favicon32: 'kernel/static/imgs/logos/bloom_64.png',
      favicon32: 'kernel/static/imgs/logos/bloom_64.png',
      favicon16: 'kernel/static/imgs/logos/bloom_32.png',
      appleTouchIcon: 'kernel/static/imgs/logos/bloom_256.png',
      maskIcon: 'kernel/static/imgs/logos/bloom_256.png',
      msTileImage: 'kernel/static/imgs/logos/bloom_256.png',
    },
  },
};
