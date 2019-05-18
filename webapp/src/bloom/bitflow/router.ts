const IndexView = () => import(/* webpackChunkName: "chunk-bitflow" */ './views/Index.vue'); // tslint:disable-line
const HistoryView = () => import(/* webpackChunkName: "chunk-bitflow" */'./views/History.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      auth: {
        layout: 'authenticated',
      },
      layout: 'unauthenticated',
      service: 'bitflow',
    },
    path: '/bitflow',
  },
  {
    component: HistoryView,
    meta: {
      auth: {
        layout: 'authenticated',
        redirect: '/bitflow',
        required: true,
      },
      service: 'bitflow',
    },
    path: '/bitflow/history',
  },
];
