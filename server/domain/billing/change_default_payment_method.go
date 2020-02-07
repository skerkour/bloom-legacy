package billing

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func ChangeDefaultPaymentMethod(ctx context.Context, user *users.User, id string) error {
	var err error
	logger := rz.FromCtx(ctx)
	var paymentMethod *PaymentMethod
	var customer *Customer
	now := time.Now().UTC()

	// validate params
	if user == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("billing.ChangeDefaultPaymentMethod: Starting transaction", rz.Err(err))
		return NewError(ErrorChangingDefaultPaymentMethod)
	}

	paymentMethod, err = FindPaymentMethodById(ctx, tx, id)
	if err != nil {
		tx.Rollback()
		return err
	}

	if paymentMethod.IsDefault {
		if err != nil {
			tx.Rollback()
			return NewError(ErrorPaymentMethodIsAlreadyDefault)
		}
	}

	customer, err = FindCustomerByPaymentMethod(ctx, tx, paymentMethod)
	if err != nil {
		tx.Rollback()
		return NewError(ErrorPaymentMethodNotFound)
	}

	if customer.GroupID != nil {
		if err = groups.CheckUserIsGroupAdmin(ctx, tx, user.ID, *customer.GroupID); err != nil {
			tx.Rollback()
			return err
		}
	} else {
		if user.ID != *customer.UserID {
			tx.Rollback()
			return NewError(ErrorPaymentMethodNotFound)
		}
	}

	oldDefaultPaymentMethod, err := FindPaymentMethodByCustomer(ctx, tx, customer, true)
	if err != nil {
		tx.Rollback()
		return NewError(ErrorPaymentMethodNotFound)
	}

	queryUpdate := "UPDATE billing_payment_methods SET is_default = $1, updated_at = $2 WHERE id = $3"

	// update oldDefaultPaymentMethod
	oldDefaultPaymentMethod.UpdatedAt = now
	oldDefaultPaymentMethod.IsDefault = false
	_, err = tx.Exec(queryUpdate, oldDefaultPaymentMethod.IsDefault, oldDefaultPaymentMethod.UpdatedAt, oldDefaultPaymentMethod.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangeDefaultPaymentMethod:updating old payment method", rz.Err(err))
		return NewError(ErrorChangingDefaultPaymentMethod)
	}

	paymentMethod.UpdatedAt = now
	paymentMethod.IsDefault = true
	_, err = tx.Exec(queryUpdate, paymentMethod.IsDefault, paymentMethod.UpdatedAt, paymentMethod.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangeDefaultPaymentMethod:updating new payment method", rz.Err(err))
		return NewError(ErrorChangingDefaultPaymentMethod)
	}

	// commit db transaction
	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangeDefaultPaymentMethod: Committing transaction", rz.Err(err))
		return NewError(ErrorChangingDefaultPaymentMethod)
	}

	return nil
}
