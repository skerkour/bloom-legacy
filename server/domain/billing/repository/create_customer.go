package repository

/*
queryCreateCustomer := `INSERT INTO billing_customers
		(id, created_at, updated_at, plan_id, stripe_customer_id, stripe_subscription_id, email, used_storage, subscription_updated_at, user_id, group_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)`
	_, err = tx.Exec(queryCreateCustomer, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.PlanID,
		ret.StripeCustomerID, ret.StripeSubscriptionID, ret.Email, ret.UsedStorage, ret.SubscriptionUpdatedAt, ret.UserID, ret.GroupID)
	if err != nil {
		logger.Error("billing.CreateCustomer: inserting new customer", rz.Err(err))
		return ret, NewError(ErrorCreatingCustomer)
	}
*/
