import SignInForm from './components/SignInForm.vue';
import RegistrationStart from './components/RegistrationStart.vue';
import RegistrationVerify from './components/RegistrationVerify.vue';
import RegistrationComplete from './components/RegistrationComplete.vue';


export default [
  // registration
  {
    component: RegistrationStart,
    meta: {
      layout: 'auth',
    },
    path: '/auth/welcome',
  },
  { path: '/auth/register', redirect: '/auth/welcome' },
  { path: '/auth/signup', redirect: '/auth/welcome' },
  { path: '/auth/sign-up', redirect: '/auth/welcome' },
  { path: '/auth/sign_up', redirect: '/auth/welcome' },
  {
    component: RegistrationVerify,
    meta: {
      layout: 'auth',
    },
    path: '/auth/welcome/verify',
  },
  {
    component: RegistrationComplete,
    meta: {
      layout: 'auth',
    },
    path: '/auth/welcome/complete',
  },

  // SignIn
  {
    component: SignInForm,
    meta: {
      layout: 'auth',
    },
    path: '/auth/sign-in',
  },
  { path: '/auth/login', redirect: '/auth/sign-in' },
  { path: '/auth/signin', redirect: '/auth/sign-in' },
  { path: '/auth/sign_in', redirect: '/auth/sign-in' },
];
