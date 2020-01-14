enum Method {
  StartRegistration = 'auth.start_registration',
  // | 'auth.registration_started'
  VerifyRegistration = 'auth.verify_registration',
  CompleteRegistration = 'auth.complete_registration',
  SignIn = 'auth.sign_in',
  SignOut = 'auth.sign_out',
}

export default Method;
