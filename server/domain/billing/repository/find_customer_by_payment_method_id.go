package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindCustomerByPaymentMethodID(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (ret billing.Customer, err error) {
	return
}
