package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) DeletePaymentMethod(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (err error) {
	return
}
