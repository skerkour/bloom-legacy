package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindDefaultPaymentMethodForCustomer(ctx context.Context, db db.Queryer, customerID uuid.UUID) (ret billing.PaymentMethod, err error) {
	return
}
