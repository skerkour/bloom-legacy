package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) RemovePaymentMethod(ctx context.Context, paymentMethodID uuid.UUID) (err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "billing.RemovePaymentMethod: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	paymentMethod, err := service.billingRepo.FindPaymentMethodByID(ctx, tx, paymentMethodID)
	if err != nil {
		tx.Rollback()
		return err
	}

	customer, err := service.billingRepo.FindCustomerByPaymentMethodID(ctx, tx, paymentMethod.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	if customer.GroupID != nil {
		err = service.groupsService.CheckUserIsGroupAdmin(ctx, tx, me.ID, *customer.GroupID)
		if err != nil {
			tx.Rollback()
			err = users.ErrPermissionDenied
			return err
		}
	} else {
		if me.ID != *customer.UserID {
			tx.Rollback()
			err = users.ErrPermissionDenied
			return
		}
	}

	customerPaymentMethodsCount, err := service.billingRepo.GetPaymentMethodsCountForCustomer(ctx, tx, customer.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	if paymentMethod.IsDefault {
		if existingPaymentMethodsCount == 1 {
			plan, err := service.billingRepo.FindPlanByID(ctx, tx, customer.PlanID)
			if err != nil {
				tx.Rollback()
				return
			}
			if plan.Product != "FREE" {
				tx.Rollback()
				err = billing.ErrRemovingDefaultPaymentMethodOnNonFreePlan
				return
			}
		} else {
			tx.Rollback()
			err = billing.ErrRemovingDefaultPaymentMethod
			return
		}
	}

	err = service.billingRepo.DeletePaymentMethod(ctx, tx, paymentMethod.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "billing.RemovePaymentMethod: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
