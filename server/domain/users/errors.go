package users

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

var (
	ErrAuthenticationRequired = errors.AuthenticationRequired("Authentication required")
	ErrMustNotBeAuthenticated = errors.PermissionDenied("Must not be authenticated")
)
