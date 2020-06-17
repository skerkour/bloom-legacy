package service

/*

currentUser := apiutil.UserFromCtx(ctx)
	var stripePlanID *string
	var stripeCustomerID *string
	var stripeSubscriptionID *string

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	customer, err := billing.FindCustomerByUserId(ctx, nil, *user.ID, false)
	if err != nil {
		return ret, gqlerrors.New(err)
	}
	plan, err := billing.FindPlanForCustomer(ctx, customer)
	if err != nil {
		return ret, gqlerrors.New(err)
	}
*/
