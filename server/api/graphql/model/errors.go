package model

import (
	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/errors"
)

func PermissionDeniedToAccessField() error {
	return api.NewError(errors.PermissionDenied("You have no right to access this field"))
}
