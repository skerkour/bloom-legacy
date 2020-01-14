package auth

func StartRegistration(params StartRegistrationParams) ([]byte, error) {
	return nil, nil
}

func VerifyRegistration(params VerifyRegistrationParams) ([]byte, error) {
	return nil, nil
}

func CompleteRegistration(params CompleteRegistrationParams) ([]byte, error) {
	return nil, nil
}

func SignOut() error {
	return nil
}

func deriveAuthKey(username, password []byte) ([]byte, error) {
	return []byte{}, nil
}
