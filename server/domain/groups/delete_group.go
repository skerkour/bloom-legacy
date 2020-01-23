package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeleteGroup(ctx context.Context, tx *sqlx.Tx, id string) twirp.Error {
	logger := rz.FromCtx(ctx)
	_ = logger

	return nil
}
