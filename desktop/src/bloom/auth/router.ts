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
    path: '/auth/registration/start',
  },
  { path: '/auth/register', redirect: '/auth/registration/start' },
  { path: '/auth/signup', redirect: '/auth/registration/start' },
  { path: '/auth/sign-up', redirect: '/auth/registration/start' },
  { path: '/auth/sign_up', redirect: '/auth/registration/start' },
  { path: '/auth/registration', redirect: '/auth/registration/start' },
  { path: '/auth/welcome', redirect: '/auth/registration/start' },
  {
    component: RegistrationVerify,
    meta: {
      layout: 'auth',
    },
    path: '/auth/registration/verify',
  },
  {
    component: RegistrationComplete,
    meta: {
      layout: 'auth',
    },
    path: '/auth/registration/complete',
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
