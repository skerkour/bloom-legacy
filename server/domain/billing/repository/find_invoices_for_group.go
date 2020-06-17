package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindInvoicesForGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret []billing.Invoice, err error) {
	ret = []billing.Invoice{}
	return
}
