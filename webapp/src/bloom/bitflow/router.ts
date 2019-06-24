const IndexView = () => import(/* webpackChunkName: "chunk-bitflow" */ './views/Index.vue'); // tslint:disable-line
const HistoryView = () => import(/* webpackChunkName: "chunk-bitflow" */'./views/History.vue'); // tslint:disable-line


export default [
  {
    component: IndexView,
    meta: {
      service: 'bitflow',
    },
    path: '/bitflow',
  },
  {
    component: HistoryView,
    meta: {
      auth: {
        redirect: '/bitflow',
        required: true,
      },
      service: 'bitflow',
    },
    path: '/bitflow/history',
  },
];
