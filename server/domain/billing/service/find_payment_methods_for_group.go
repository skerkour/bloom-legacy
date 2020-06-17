package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) FindPaymentMethodsForGroup(ctx context.Context, groupID uuid.UUID) (ret []billing.PaymentMethod, err error) {
	return
}

/*

	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdmin(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	paymentMethods, err := billing.FindPaymentMethodsByGroupId(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

*/
