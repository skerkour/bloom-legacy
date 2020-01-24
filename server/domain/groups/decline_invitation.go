package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeclineInvitation(ctx context.Context, tx *sqlx.Tx, user users.User, invitation Invitation) twirp.Error {
	logger := rz.FromCtx(ctx)
	_ = logger
	return nil
}
