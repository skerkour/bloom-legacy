package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) ChangeDefaultPaymentMethod(ctx context.Context, paymentMethodID uuid.UUID) (ret billing.PaymentMethod, err error) {
	return
}
