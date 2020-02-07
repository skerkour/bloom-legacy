package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func ChangePlan(ctx context.Context, tx *sqlx.Tx, actor *users.User, userId, groupId *string, planId string) error {
	logger := rz.FromCtx(ctx)
	// if userId != actor && !actor.IsAdmin
	// or actor is not groupAdmin or admin

	// validate params
	if actor == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
	}
	return nil
}
