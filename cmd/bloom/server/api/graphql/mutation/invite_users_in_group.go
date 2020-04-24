package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
	"gitlab.com/bloom42/lily/rz"
)

func (r *Resolver) InviteUsersInGroup(ctx context.Context, input model.InviteUsersInGroupInput) (*model.Group, error) {
	var ret *model.Group
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(groups.NewError(groups.ErrorInvitingUsers))
	}

	var group groups.Group

	queryGetGroup := "SELECT * FROM groups WHERE id = $1"
	err = tx.Get(&group, queryGetGroup, input.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("fetching group", rz.Err(err),
			rz.String("group.id", input.ID.String()))
		return ret, gqlerrors.New(groups.NewError(groups.ErrorGroupNotFound))
	}

	err = groups.InviteUsers(ctx, tx, *currentUser, group, input.Users)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("Committing transaction", rz.Err(err))
		return ret, gqlerrors.New(groups.NewError(groups.ErrorInvitingUsers))
	}

	ret = &model.Group{
		ID:          &group.ID,
		CreatedAt:   &group.CreatedAt,
		Name:        group.Name,
		Description: group.Description,
	}

	return ret, nil
}
