package mutation

import (
	"context"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

func (r *Resolver) RemoveGroupMembers(ctx context.Context, input model.RemoveGroupMembersInput) (*model.Group, error) {
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
		logger.Error("groups.RemoveMembers: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError(groups.ErrorRemovingMembersMsg)
	}

	var group groups.Group

	queryGetGroup := "SELECT * FROM groups WHERE id = $1"
	err = tx.Get(&group, queryGetGroup, params.GroupId)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.DeleteGroup: fetching group", rz.Err(err),
			rz.String("id", params.GroupId))
		return ret, twirp.NewError(twirp.NotFound, "Group not found.")
	}

	twerr := groups.RemoveMembers(ctx, tx, *apiCtx.AuthenticatedUser, group, params.Usernames)
	if twerr != nil {
		tx.Rollback()
		return ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("groups.RemoveMembers: Committing transaction", rz.Err(err))
		return ret, twirp.InternalError(groups.ErrorRemovingMembersMsg)
	}

	return ret, nil
}
