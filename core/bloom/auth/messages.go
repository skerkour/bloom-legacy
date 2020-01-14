package auth

type SignInParams struct {
}

type StartRegistrationParams struct {
}

type VerifyRegistrationParams struct {
}

type CompleteRegistrationParams struct {
}

type Session struct {
	ID    string `json:"id"`
	Token string `json:"token"`
}
