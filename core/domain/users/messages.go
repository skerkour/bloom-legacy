package users

type SignInParams struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type StartRegistrationParams struct {
	DisplayName string `json:"displayName"`
	Email       string `json:"email"`
}

type VerifyRegistrationParams struct {
	ID   string `json:"id"`
	Code string `json:"code"`
}

type CompleteRegistrationParams struct {
	ID       string `json:"id"`
	Username string `json:"username"`
	Password string `json:"password"`
}

type RegistrationSendNewCodeParams struct {
	ID string `json:"id"`
}

type RevokeSessionParams struct {
	ID string `json:"id"`
}
