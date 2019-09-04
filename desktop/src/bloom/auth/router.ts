import SignInForm from './components/SignInForm.vue';
import RegisterForm from './components/RegisterForm.vue';


export default [
  // registration
  {
    component: RegisterForm,
    meta: {
      auth: {
        forbidden: true,
      },
      layout: 'auth',
    },
    path: '/register',
  },
  { path: '/signup', redirect: '/register' },
  { path: '/sign-up', redirect: '/register' },
  { path: '/sign_up', redirect: '/register' },

  // SignIn
  {
    component: SignInForm,
    meta: {
      auth: {
        forbidden: true,
      },
      layout: 'auth',
    },
    path: '/sign-in',
  },
  { path: '/login', redirect: '/sign-in' },
  { path: '/signin', redirect: '/sign-in' },
  { path: '/sign_in', redirect: '/sign-in' },
];
