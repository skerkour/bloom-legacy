package service

import (
	"context"

	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) RemovePaymentMethod(ctx context.Context, paymentMethodID uuid.UUID) (err error) {
	return
}
