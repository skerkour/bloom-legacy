package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

// AddPaymentMethod is used by users to add a payment method to their account or group
func (resolver *Resolver) AddPaymentMethod(ctx context.Context, input model.AddPaymentMethodInput) (ret *model.PaymentMethod, err error) {
	params := billing.AddPaymentMethodParams{
		StripeID: input.StripeID,
		GroupID:  input.GroupID,
	}
	paymentMethod, err := resolver.billingService.AddPaymentMethod(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.PaymentMethod{
		ID:                  paymentMethod.ID,
		CreatedAt:           paymentMethod.CreatedAt,
		CardLast4:           paymentMethod.CardLast4,
		CardExpirationMonth: int(paymentMethod.CardExpirationMonth),
		CardExpirationYear:  int(paymentMethod.CardExpirationYear),
		IsDefault:           paymentMethod.IsDefault,
	}
	return
}
