package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindInvoicesForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret []billing.Invoice, err error) {
	ret = []billing.Invoice{}
	return
}
