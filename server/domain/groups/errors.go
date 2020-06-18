package groups

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

var (
	ErrInvitationNotFound = errors.NotFound("Invitation not found.")
	ErrAdminRoleRequired  = errors.NotFound("Admin role is required.")
)
