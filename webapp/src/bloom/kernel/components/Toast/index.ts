/* tslint:disable */
import Toast from './Toast.vue'

function init(Vue: any, options: any = {}) {
  let cmp: any = null;
  const property = options.property || '$toast';

  function createCmp(options: any) {
    cmp = new Vue(Toast)
    Object.assign(cmp, Vue.prototype[property].options, options)
    document.body.appendChild(cmp.$mount().$el)
  }

  function show(message: string, options: any = {}) {
    // TODO: enqueue
    if (cmp) {
      cmp.close()
      Vue.nextTick(() => {
        cmp = null
        show(message, options)
      })
      return
    }

    options.message = message
    return createCmp(options)
  }

  function shorts(options: any) {
    const colors = ['success', 'info', 'error', 'warning']
    let methods: any = {}

    colors.forEach(color => {
      methods[color] = (message: string, options: any) => show(message, { color, ...options })
    })
    if (options.shorts) {
      for (let key in options.shorts) {
        let localOptions = options.shorts[key]
        methods[key] = (message: string, options: any) => show(message, { ...localOptions, ...options })
      }
    }

    return methods
  }

  Vue.prototype[property] = Object.assign(show, {
    options,
    ...shorts(options)
  })
}

// if (typeof window !== 'undefined' && window.Vue) {
//   window.Vue.use(init)
// }

export default init
