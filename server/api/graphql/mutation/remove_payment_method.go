package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// RemovePaymentMethod is used to remove a payment method
func (resolver *Resolver) RemovePaymentMethod(ctx context.Context, input model.RemovePaymentMethodInput) (ret bool, err error) {
	err = resolver.billingService.RemovePaymentMethod(ctx, input.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
