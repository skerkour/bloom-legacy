package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (service *BillingService) AddPaymentMethod(ctx context.Context, params billing.AddPaymentMethodParams) (ret billing.PaymentMethod, err error) {
	return
}
