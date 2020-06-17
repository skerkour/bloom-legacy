package users

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

// Service is the application service for the users subdomain
type Service interface {
	// Commands
	StartRegistration(ctx context.Context, params StartRegistrationParams) (newPendingUserID uuid.UUID, err error)
	CompleteRegistration(ctx context.Context, params CompleteRegistrationParams) (user User, session Session, token string, err error)
	SignIn(ctx context.Context, params SignInParams) (user User, session Session, token string, err error)
	EnableUser(ctx context.Context, userID uuid.UUID) error
	RevokeSession(ctx context.Context, sessionID uuid.UUID) error
	SendNewRegistrationCode(ctx context.Context, pendingUserID uuid.UUID) error
	UpdateUserProfile(ctx context.Context, params UpdateUserProfileParams) (User, error)
	UpdateUserState(ctx context.Context, db db.Queryer, user User, newState int64) error
	VerifyPendingUser(ctx context.Context, params VerifyPendingUserParams) (err error)
	DisableUser(ctx context.Context, userID uuid.UUID) error

	// Queries
	FindAllUsers(ctx context.Context) ([]User, error)
	VerifySessionToken(ctx context.Context, token string) (User, Session, error)
	Me(ctx context.Context) (User, error)
	FindUserByUsername(ctx context.Context, username string) (User, error)
	FindSessionsForUser(ctx context.Context, userID uuid.UUID) ([]Session, error)
}

// SignInParams are the parameters for SignIn
type SignInParams struct {
	Username  string
	AuthKey   []byte
	Device    SessionDevice
	IPAddress string
}

// CompleteRegistrationParams are the parameters for CompleteRegistration
type CompleteRegistrationParams struct {
	PendingUserID       uuid.UUID
	Username            string
	AuthKey             []byte
	Device              SessionDevice
	PublicKey           []byte
	EncryptedPrivateKey []byte
	PrivateKeyNonce     []byte
	EncryptedMasterKey  []byte
	MasterKeyNonce      []byte
}

// StartRegistrationParams are parameters for StartRegistration
type StartRegistrationParams struct {
	DisplayName string
	Email       string
}

// UpdateUserProfileParams are the parameters for UpdateProfile
type UpdateUserProfileParams struct {
	UserID      *uuid.UUID `json:"id"`
	DisplayName *string    `json:"displayName"`
	Bio         *string    `json:"bio"`
	FirstName   *string    `json:"firstName"`
	LastName    *string    `json:"lastName"`
}

// VerifyPendingUserParams are the parameters for VerifyPendingUser
type VerifyPendingUserParams struct {
	PendingUserID uuid.UUID
	Code          string
}
