package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) FindPaymentMethodsForUser(ctx context.Context, userID uuid.UUID) (ret []billing.PaymentMethod, err error) {
	return
}

/*
	var ret *PaymentMethodConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	paymentMethods, err := billing.FindPaymentMethodsByUserId(ctx, nil, uuid.UUID(*user.ID).String())
	if err != nil {
		return ret, gqlerrors.New(err)
	}
*/
