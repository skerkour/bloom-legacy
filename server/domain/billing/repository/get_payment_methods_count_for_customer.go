package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) GetPaymentMethodsCountForCustomer(ctx context.Context, db db.Queryer, customerID uuid.UUID) (ret int64, err error) {
	return
}

/*
	queryCountExistingPaymentMethods := "SELECT COUNT(*) FROM billing_payment_methods WHERE customer_id = $1"
	err = tx.Get(&existingPaymentMethodsCount, queryCountExistingPaymentMethods, customer.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.RemovePaymentMethod: error fetching existing payment methods counts", rz.Err(err))
		return NewError(ErrorRemovingPaymentMethod)
	}
*/
