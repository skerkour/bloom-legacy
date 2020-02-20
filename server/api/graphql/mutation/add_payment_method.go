package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (r *Resolver) AddPaymentMethod(ctx context.Context, input model.AddPaymentMethodInput) (*model.PaymentMethod, error) {
	var ret *model.PaymentMethod
	var err error
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	paymentMethod, err := billing.AddPaymentMethod(ctx, currentUser, input.StripeID, input.GroupID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &model.PaymentMethod{
		ID:                  paymentMethod.ID,
		CreatedAt:           paymentMethod.CreatedAt,
		CardLast4:           paymentMethod.CardLast4,
		CardExpirationMonth: int(paymentMethod.CardExpirationMonth),
		CardExpirationYear:  int(paymentMethod.CardExpirationYear),
		IsDefault:           paymentMethod.IsDefault,
	}
	return ret, nil
}
