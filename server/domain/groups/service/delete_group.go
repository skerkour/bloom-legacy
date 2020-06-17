package service

import (
	"context"

	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) DeleteGroup(ctx context.Context, groupID uuid.UUID) (err error) {
	return
}

/*

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.DeleteGroup: Starting transaction", rz.Err(err))
		err = gqlerrors.New(groups.NewError(groups.ErrorDeletingGroup))
		return
	}

	err = billing.DetachCustomer(ctx, tx, nil, &input.ID)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	err = groups.DeleteGroup(ctx, tx, currentUser, input.ID)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	err = objects.DeleteGroupObjects(ctx, tx, input.ID)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.DeleteGroup: Committing transaction", rz.Err(err))
		err = gqlerrors.New(groups.NewError(groups.ErrorDeletingGroup))
		return
	}
*/
