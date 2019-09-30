import SignInForm from './components/SignInForm.vue';
import RegisterStart from './components/RegisterStart.vue';
import RegisterVerify from './components/RegisterVerify.vue';


export default [
  // registration
  {
    component: RegisterStart,
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
    component: RegisterVerify,
    meta: {
      layout: 'auth',
    },
    path: '/auth/welcome/verify',
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
