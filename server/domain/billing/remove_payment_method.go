package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func RemovePaymentMethod(ctx context.Context, user *users.User, id string) error {
	var err error
	logger := rz.FromCtx(ctx)
	var paymentMethod *PaymentMethod
	var customer *Customer
	existingPaymentMethodsCount := 0
	var plan *Plan

	// validate params
	if user == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return NewError(ErrorRemovingPaymentMethod)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("billing.RemovePaymentMethod: Starting transaction", rz.Err(err))
		return NewError(ErrorRemovingPaymentMethod)
	}

	paymentMethod, err = FindPaymentMethodById(ctx, tx, id)
	if err != nil {
		tx.Rollback()
		return err
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

	// find count of existing payment method for this customer
	queryCountExistingPaymentMethods := "SELECT COUNT(*) FROM billing_payment_methods WHERE customer_id = $1"
	err = tx.Get(&existingPaymentMethodsCount, queryCountExistingPaymentMethods, customer.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.RemovePaymentMethod: error fetching existing payment methods counts", rz.Err(err))
		return NewError(ErrorRemovingPaymentMethod)
	}

	if paymentMethod.IsDefault {
		if existingPaymentMethodsCount == 1 {
			plan, err = FindPublicPlanById(ctx, tx, customer.PlanID)
			if err != nil {
				tx.Rollback()
				return err
			}
			if plan.Product != "FREE" {
				tx.Rollback()
				return NewError(ErrorRemovingDefaultPaymentMethodOnNonFreePlan)
			}
		} else {
			tx.Rollback()
			return NewError(ErrorRemovingDefaultPaymentMethod)
		}
	}

	// delete payment method
	queryDelete := "DELETE FROM billing_payment_methods WHERE id = $1"
	_, err = tx.Exec(queryDelete, paymentMethod.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.RemovePaymentMethod: deleting payment method", rz.Err(err))
		return NewError(ErrorRemovingPaymentMethod)
	}

	// commit db transaction
	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("billing.RemovePaymentMethod: Committing transaction", rz.Err(err))
		return NewError(ErrorRemovingPaymentMethod)
	}

	return nil
}
