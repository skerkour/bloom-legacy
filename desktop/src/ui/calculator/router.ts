const Calculator = () => import(/* webpackChunkName: "chunk-calculator" */ './views/Calculator.vue');

export default [
  {
    component: Calculator,
    path: '/calculator',
  },
];
