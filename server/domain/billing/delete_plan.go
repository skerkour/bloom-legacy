package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeletePlan(ctx context.Context, user *users.User, planId string) error {
	var err error
	logger := rz.FromCtx(ctx)

	// validate params
	if user == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return NewError(ErrorDeletingPlan)
	}
	if !user.IsAdmin {
		return NewError(ErrorAdminRoleRequired)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("billing.DeletePlan: Starting transaction", rz.Err(err))
		return NewError(ErrorDeletingPlan)
	}

	defaultPlan, err := FindDefaultPlan(ctx, tx)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.DeletePlan: finding default plan", rz.Err(err))
		return err
	}

	if defaultPlan.ID == planId {
		tx.Rollback()
		return NewError(ErrorCantDeleteDefaultPlan)
	}

	subscribersCount, err := GetSubscribersCountForPlanIdTx(ctx, tx, planId)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.DeletePlan: finding subscribers count", rz.Err(err))
		return err
	}

	if subscribersCount != 0 {
		tx.Rollback()
		return NewError(ErrorCantDeletePlanWithSubscribers)
	}

	// delete plan
	queryDeleteGroup := "DELETE FROM billing_plans WHERE id = $1"
	_, err = tx.Exec(queryDeleteGroup, planId)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.DeletePlan: deleting plan", rz.Err(err))
		return NewError(ErrorDeletingPlan)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("billing.DeletePlan: Committing transaction", rz.Err(err))
		return NewError(ErrorDeletingPlan)
	}

	return err
}
