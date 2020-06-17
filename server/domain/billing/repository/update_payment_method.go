package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (repo *BillingRepository) UpdatePaymentMethod(ctx context.Context, db db.Queryer, paymentMethod billing.PaymentMethod) (err error) {
	return
}
