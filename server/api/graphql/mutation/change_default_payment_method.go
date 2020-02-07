package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (r *Resolver) ChangeDefaultPaymentMethod(ctx context.Context, input model.ChangeDefaultPaymentMethodInput) (*model.PaymentMethod, error) {
	var ret *model.PaymentMethod
	var err error
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	newDefaultPaymentMethod, err := billing.ChangeDefaultPaymentMethod(ctx, currentUser, input.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
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
