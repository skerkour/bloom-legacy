package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// ChangeDefaultPaymentMethod is used by users to change their default payment mehtod
func (resolver *Resolver) ChangeDefaultPaymentMethod(ctx context.Context, input model.ChangeDefaultPaymentMethodInput) (ret *model.PaymentMethod, err error) {
	newDefaultPaymentMethod, err := resolver.billingService.ChangeDefaultPaymentMethod(ctx, input.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.PaymentMethod{
		ID:                  newDefaultPaymentMethod.ID,
		CreatedAt:           newDefaultPaymentMethod.CreatedAt,
		CardLast4:           newDefaultPaymentMethod.CardLast4,
		CardExpirationMonth: int(newDefaultPaymentMethod.CardExpirationMonth),
		CardExpirationYear:  int(newDefaultPaymentMethod.CardExpirationYear),
	}
	return ret, nil
}
