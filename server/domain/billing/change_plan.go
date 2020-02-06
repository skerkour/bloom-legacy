package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

func ChangePlan(ctx context.Context, tx *sqlx.Tx, actor *users.User, userId, groupId *string, planId string) error {
	// if userId != actor && !actor.IsAdmin
	// or actor is not groupAdmin or admin
	return nil
}
