package users

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

var (
	ErrAuthenticationRequired = errors.AuthenticationRequired("Permission denied: authentication required")
	ErrMustNotBeAuthenticated = errors.PermissionDenied("Permission denied: must not be authenticated")
	ErrPermissionDenied       = errors.PermissionDenied("Permission denied")
)
