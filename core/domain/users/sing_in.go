package users

func SignIn(params SignInParams) (Session, error) {
	// client := users.NewUsersProtobufClient("http://localhost:8000", &http.Client{})

	// authKey := deriveAuthKey([]byte(params.Username), []byte(params.Password))
	// if authKey == nil {
	// 	return Session{}, errors.New("Error deriving auth key")
	// }
	// rpcParams := users.SignInParams{
	// 	Username: params.Username,
	// 	AuthKey:  authKey,
	// }

	// session, err := client.SignIn(context.Background(), &rpcParams)
	// if err != nil {
	// 	return Session{}, err
	// }
	// return Session{
	// 	ID:    session.Id,
	// 	Token: session.Token,
	// }, nil
	return Session{}, nil
}
