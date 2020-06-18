package groups

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

var (
	ErrInvitationNotFound                          = errors.NotFound("Invitation not found.")
	ErrAdminRoleRequired                           = errors.PermissionDenied("Admin role is required.")
	ErrAtLeastOneAdministratorShouldRemainsInGroup = errors.InvalidArgument("At least one administrator should remain in group.")
)
