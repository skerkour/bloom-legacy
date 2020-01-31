package handler

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/libs/rz-go"
)

func (handler Handler) CancelInvitation(ctx context.Context, params *rpc.CancelInvitationParams) (*rpc.Empty, error) {
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
		logger.Error("groups.CancelInvitation: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError(groups.ErrorCancelingInvitationMsg)
	}

	var invitation groups.Invitation

	queryGetInvitation := "SELECT * FROM groups_invitations WHERE id = $1 FOR UPDATE"
	err = tx.Get(&invitation, queryGetInvitation, params.InvitationId)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.CancelInvitation: fetching invitation", rz.Err(err),
			rz.String("invitation_id", params.InvitationId))
		return ret, twirp.NewError(twirp.NotFound, "Invitation not found.")
	}

	twerr := groups.CancelInvitation(ctx, tx, *apiCtx.AuthenticatedUser, invitation)
	if twerr != nil {
		tx.Rollback()
		return ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("groups.CancelInvitation: Committing transaction", rz.Err(err))
		return ret, twirp.InternalError(groups.ErrorCancelingInvitationMsg)
	}

	return ret, nil
}
