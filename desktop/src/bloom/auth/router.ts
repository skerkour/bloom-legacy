import SignInForm from './components/SignInForm.vue';
import RegisterForm from './components/RegisterForm.vue';

export default [
  // registration
  {
    component: RegisterForm,
    meta: {
      layout: 'auth',
    },
    path: '/auth/register',
  },
  { path: '/auth/signup', redirect: '/auth/register' },
  { path: '/auth/sign-up', redirect: '/auth/register' },
  { path: '/auth/sign_up', redirect: '/auth/register' },

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
