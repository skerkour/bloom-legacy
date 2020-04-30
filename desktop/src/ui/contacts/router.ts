const Contacts = () => import(/* webpackChunkName: "chunk-contacts" */ './views/contacts.vue');

export default [
  {
    component: Contacts,
    path: '/contacts',
  },
];
