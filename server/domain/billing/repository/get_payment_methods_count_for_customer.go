package repository

/*
	queryCountExistingPaymentMethods := "SELECT COUNT(*) FROM billing_payment_methods WHERE customer_id = $1"
	err = tx.Get(&existingPaymentMethodsCount, queryCountExistingPaymentMethods, customer.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.RemovePaymentMethod: error fetching existing payment methods counts", rz.Err(err))
		return NewError(ErrorRemovingPaymentMethod)
	}
*/
