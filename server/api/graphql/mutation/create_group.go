package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/libs/rz-go"
)

func (r *Resolver) CreateGroup(ctx context.Context, input model.CreateGroupInput) (*model.Group, error) {
	var ret *model.Group
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.CreateGroup: Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(groups.NewError(groups.ErrorCreatingGroup))
	}

	newGroup, err := groups.CreateGroup(ctx, tx, *currentUser, input.Name, input.Description)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	// create customer profile
	_, err = billing.CreateCustomer(ctx, tx, currentUser, nil, &newGroup.ID)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.CreateGroup: Committing transaction", rz.Err(err))
		return ret, gqlerrors.New(groups.NewError(groups.ErrorCreatingGroup))
	}

	ret = &model.Group{
		ID:          &newGroup.ID,
		Name:        newGroup.Name,
		Description: newGroup.Description,
		CreatedAt:   &newGroup.CreatedAt,
	}
	return ret, nil
}
