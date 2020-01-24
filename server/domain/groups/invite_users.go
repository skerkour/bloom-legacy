package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)


func InviteUsers(ctx context.Context, tx *sqlx.Tx, user users.User, group Group, usernames []string) twirp.Error {
	logger := rz.FromCtx(ctx)
	_ = logger

	if twerr := validateInviteUsers(ctx, tx, user, group, usernames); twerr != nil {
		return twerr
	}

	return nil
}


// TODO: check that user is not already in group or awaiting invitations
func validateInviteUsers(ctx context.Context, tx *sqlx.Tx, user users.User, group Group, usernames []string) twirp.Error {
	logger := rz.FromCtx(ctx)
	_ = logger

	// chack that user inviting is admin
	if twerr := checkUserIsGroupAdmin(ctx, tx, user.ID, group.ID); twerr != nil {
		return twerr
	}

	return nil
}
