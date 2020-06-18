package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindInvoicesForGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret []billing.Invoice, err error) {
	ret = []billing.Invoice{}
	query := `SELECT billing_invoices.* FROM billing_invoices
		INNER JOIN billing_customers ON billing_invoices.customer_id = billing_customers.id
		WHERE billing_customers.group_id = $1 ORDER BY created_at DESC`

	err = db.Select(ctx, &ret, query, groupID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.FindInvoicesForGroup: finding invoices"
		logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", groupID))
		err = errors.Internal(errMessage, err)
	}
	return
}
