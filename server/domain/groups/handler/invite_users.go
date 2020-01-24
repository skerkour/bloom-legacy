package handler

import (
	"context"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/libs/rz-go"
)

func (handler Handler) InviteUsers(ctx context.Context, params *rpc.InviteUsersParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return ret, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedUser == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return ret, twerr
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.InviteUsers: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError(groups.ErrorInvitingUsersMsg)
	}

	var group groups.Group

	queryGetGroup := "SELECT * FROM groups WHERE id = $1"
	err = tx.Get(&group, queryGetGroup, params.GroupId)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUsers: fetching group", rz.Err(err),
			rz.String("id", params.GroupId))
		return ret, twirp.NewError(twirp.NotFound, "Group not found.")
	}

	twerr := groups.InviteUsers(ctx, tx, *apiCtx.AuthenticatedUser, group, params.Usernames)
	if twerr != nil {
		tx.Rollback()
		return ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUsers: Committing transaction", rz.Err(err))
		return ret, twirp.InternalError(groups.ErrorInvitingUsersMsg)
	}

	return ret, nil
}
