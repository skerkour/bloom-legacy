package users

import "gitlab.com/bloom42/gobox/uuid"

type SignInParams struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type StartRegistrationParams struct {
	DisplayName string `json:"displayName"`
	Email       string `json:"email"`
}

type VerifyRegistrationParams struct {
	ID   uuid.UUID `json:"id"`
	Code string    `json:"code"`
}

type CompleteRegistrationParams struct {
	ID       uuid.UUID `json:"id"`
	Username string    `json:"username"`
	Password string    `json:"password"`
}

type RegistrationSendNewCodeParams struct {
	ID uuid.UUID `json:"id"`
}

type RevokeSessionParams struct {
	ID uuid.UUID `json:"id"`
}

type FetchUserParams struct {
	Username string `json:"username"`
}

type EnableUserParams struct {
	ID uuid.UUID `json:"id"`
}

type DisableUserParams struct {
	ID uuid.UUID `json:"id"`
}
