package billing

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func ChangeDefaultPaymentMethod(ctx context.Context, user *users.User, id string) (*PaymentMethod, error) {
	var err error
	logger := rz.FromCtx(ctx)
	var ret *PaymentMethod
	var customer *Customer
	now := time.Now().UTC()

	// validate params
	if user == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return ret, NewError(ErrorChangingDefaultPaymentMethod)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("billing.ChangeDefaultPaymentMethod: Starting transaction", rz.Err(err))
		return ret, NewError(ErrorChangingDefaultPaymentMethod)
	}

	ret, err = FindPaymentMethodById(ctx, tx, id)
	if err != nil {
		tx.Rollback()
		return ret, err
	}

	if ret.IsDefault {
		if err != nil {
			tx.Rollback()
			return ret, NewError(ErrorPaymentMethodIsAlreadyDefault)
		}
	}

	customer, err = FindCustomerByPaymentMethod(ctx, tx, ret)
	if err != nil {
		tx.Rollback()
		return ret, NewError(ErrorPaymentMethodNotFound)
	}

	if customer.GroupID != nil {
		if err = groups.CheckUserIsGroupAdmin(ctx, tx, user.ID, *customer.GroupID); err != nil {
			tx.Rollback()
			return ret, err
		}
	} else {
		if user.ID != *customer.UserID {
			tx.Rollback()
			return ret, NewError(ErrorPaymentMethodNotFound)
		}
	}

	oldDefaultPaymentMethod, err := FindPaymentMethodByCustomer(ctx, tx, customer, true)
	if err != nil {
		tx.Rollback()
		return ret, NewError(ErrorPaymentMethodNotFound)
	}

	queryUpdate := "UPDATE billing_payment_methods SET is_default = $1, updated_at = $2 WHERE id = $3"

	// update oldDefaultPaymentMethod
	oldDefaultPaymentMethod.UpdatedAt = now
	oldDefaultPaymentMethod.IsDefault = false
	_, err = tx.Exec(queryUpdate, oldDefaultPaymentMethod.IsDefault, oldDefaultPaymentMethod.UpdatedAt, oldDefaultPaymentMethod.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangeDefaultPaymentMethod: updating old payment method", rz.Err(err))
		return ret, NewError(ErrorChangingDefaultPaymentMethod)
	}

	ret.UpdatedAt = now
	ret.IsDefault = true
	_, err = tx.Exec(queryUpdate, ret.IsDefault, ret.UpdatedAt, ret.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangeDefaultPaymentMethod: updating new payment method", rz.Err(err))
		return ret, NewError(ErrorChangingDefaultPaymentMethod)
	}

	// commit db transaction
	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangeDefaultPaymentMethod: Committing transaction", rz.Err(err))
		return ret, NewError(ErrorChangingDefaultPaymentMethod)
	}

	return ret, nil
}
