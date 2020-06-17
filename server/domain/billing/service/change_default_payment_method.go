package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) ChangeDefaultPaymentMethod(ctx context.Context, paymentMethodID uuid.UUID) (ret billing.PaymentMethod, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)
	now := time.Now().UTC()

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "billing.ChangeDefaultPaymentMethod: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	ret, err = service.billingRepo.FindPaymentMethodByID(ctx, tx, paymentMethodID)
	if err != nil {
		tx.Rollback()
		return
	}

	if ret.IsDefault {
		tx.Rollback()
		err = billing.ErrPaymentMethodIsAlreadyDefault
		return
	}

	customer, err := service.billingRepo.FindCustomerByPaymentMethodID(ctx, tx, paymentMethodID)
	if err != nil {
		tx.Rollback()
		return
	}

	if customer.GroupID != nil {
		err = service.groupsService.CheckUserIsGroupAdmin(ctx, tx, user.ID, *customer.GroupID)
		if err != nil {
			tx.Rollback()
			return
		}
	} else {
		if user.ID != *customer.UserID {
			tx.Rollback()
			err = billing.ErrPaymentMethodNotFound
			return
		}
	}

	oldDefaultPaymentMethod, err := service.billingRepo.FindDefaultPaymentMethodForCustomer(ctx, tx, customer.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	oldDefaultPaymentMethod.UpdatedAt = now
	oldDefaultPaymentMethod.IsDefault = false
	err = service.billingRepo.UpdatePaymentMethod(ctx, tx, oldDefaultPaymentMethod)
	if err != nil {
		tx.Rollback()
		return
	}

	ret.UpdatedAt = now
	ret.IsDefault = true
	err = service.billingRepo.UpdatePaymentMethod(ctx, tx, ret)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "billing.ChangeDefaultPaymentMethod: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	return
}
