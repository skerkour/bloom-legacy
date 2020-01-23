package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/libs/rz-go"
)

func validateCreateGroup(name, description string) twirp.Error {
	var err error

	if err = validator.GroupName(name); err != nil {
		return twirp.InvalidArgumentError("name", err.Error())
	}

	if err = validator.GroupDescription(description); err != nil {
		return twirp.InvalidArgumentError("description", err.Error())
	}

	return nil
}

func CreateGroup(ctx context.Context, tx *sqlx.Tx, name, description string) (Group, twirp.Error) {
	logger := rz.FromCtx(ctx)
	ret := Group{}
	_ = logger

	twerr := validateCreateGroup(name, description)
	if twerr != nil {
		return ret, twerr
	}

	return ret, nil
}
