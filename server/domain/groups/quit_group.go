package groups

import (
	"context"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func QuitGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group Group) twirp.Error {
	logger := rz.FromCtx(ctx)
	_ = logger
	// TODO: check that admin is the last to quit
	return nil
}
