package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) DeletePlan(ctx context.Context, planID uuid.UUID) (err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	if !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "billing.DeletePlan: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	defaultPlan, err := service.billingRepo.FindDefaultPlan(ctx, tx)
	if err != nil {
		tx.Rollback()
		return
	}

	if defaultPlan.ID == planID {
		tx.Rollback()
		err = billing.ErrCantDeleteDefaultPlan
		return
	}

	subscribersCount, err := service.billingRepo.GetSubscribersCountForPlan(ctx, tx, planID)
	if err != nil {
		tx.Rollback()
		return
	}

	if subscribersCount != 0 {
		tx.Rollback()
		err = billing.ErrCantDeletePlanWithSubscribers
		return
	}

	err = service.billingRepo.DeletePlan(ctx, tx, planID)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "billing.DeletePlan: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	return
}
