package model

import (
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/errors"
)

func PermissionDeniedToAccessField() error {
	return gqlerrors.New(errors.New(errors.PermissionDenied, "You have no right to access this field"))
}
