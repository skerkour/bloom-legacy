package users

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

var (
	ErrAuthenticationRequired             = errors.AuthenticationRequired("Permission denied: authentication required")
	ErrMustNotBeAuthenticated             = errors.PermissionDenied("Permission denied: must not be authenticated")
	ErrPermissionDenied                   = errors.PermissionDenied("Permission denied")
	ErrInvalidSession                     = errors.PermissionDenied("Session is not valid")
	ErrPendingUserNotVerified             = errors.PermissionDenied("User is not verified")
	ErrEmailAlreadyInUse                  = errors.AlreadyExists("An account with this email already exsits. Please change and try again.")
	ErrUsernameAlreadyInUse               = errors.AlreadyExists("An account with this username already exsits. Please change and try again.")
	ErrInvalidUsernamePasswordCombination = errors.InvalidArgument("Invalid Username / Password combination")
	ErrCantDowngradeUserState             = errors.InvalidArgument("You can't downgrade user's state")
)
