package service

/*

	var ret *BillingSubscription
	currentUser := apiutil.UserFromCtx(ctx)
	var stripePlanID *string
	var stripeCustomerID *string
	var stripeSubscriptionID *string
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdmin(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	customer, err := billing.FindCustomerByGroupID(ctx, nil, *group.ID, false)
	if err != nil {
		return ret, gqlerrors.New(err)
	}
	plan, err := billing.FindPlanForCustomer(ctx, customer)
	if err != nil {
		return ret, gqlerrors.New(err)
	}
*/
