package auth

type SignInParams struct {
	Username string `json:"username"`
	Password string `json:"password"`
	// AuthKey  string `json:"auth_key"`
}

type StartRegistrationParams struct {
	DisplayName string `json:"display_name"`
	Email       string `json:"email"`
}

type RegistrationStarted struct {
	ID string `json:"id"`
}

type VerifyRegistrationParams struct {
	ID   string `json:"id"`
	Code string `json:"code"`
}

type CompleteRegistrationParams struct {
	// ID       string `json:"id"`
	// Username string `json:"username"`
	// AuthKey  string `json:"auth_key"`
	ID       string `json:"id"`
	Username string `json:"username"`
	Password string `json:"password"`
}

type RegistrationSendNewCodeParams struct {
	ID string `json:"id"`
}

type Session struct {
	ID    string `json:"id"`
	Token string `json:"token"`
}

type RevokeSessionParams struct {
	ID string `json:"id"`
}
