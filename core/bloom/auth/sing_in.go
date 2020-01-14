package auth

func SignIn(params SignInParams) (Session, error) {
	return Session{
		ID:    "myRandomID",
		Token: "myRandomTOken",
	}, nil
}
