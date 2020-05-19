package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/rz"
)

// CreateGroup is used to create a group
func (r *Resolver) CreateGroup(ctx context.Context, input model.CreateGroupInput) (ret *model.Group, err error) {
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.CreateGroup: Starting transaction", rz.Err(err))
		err = gqlerrors.New(groups.NewError(groups.ErrorCreatingGroup))
		return
	}

	params := groups.CreateGroupParams{
		Name:          input.Name,
		Description:   input.Description,
		UsersToInvite: input.UsersToInvite,
	}
	newGroup, err := groups.CreateGroup(ctx, tx, currentUser, params)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	// create customer profile
	_, err = billing.CreateCustomer(ctx, tx, currentUser, nil, &newGroup.ID)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.CreateGroup: Committing transaction", rz.Err(err))
		err = gqlerrors.New(groups.NewError(groups.ErrorCreatingGroup))
		return
	}

	ret = &model.Group{
		ID:          &newGroup.ID,
		Name:        newGroup.Name,
		Description: newGroup.Description,
		CreatedAt:   &newGroup.CreatedAt,
	}
	return ret, nil
}
