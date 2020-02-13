const Contacts = () => import(/* webpackChunkName: "chunk-contacts" */ './views/Contacts.vue');

export default [
  {
    component: Contacts,
    path: '/contacts',
  },
];
