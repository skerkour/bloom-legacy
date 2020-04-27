package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

func FindInvoicesByGroupId(ctx context.Context, tx *sqlx.Tx, groupID uuid.UUID) ([]Invoice, error) {
	ret := []Invoice{}
	var err error
	logger := rz.FromCtx(ctx)

	query := `SELECT billing_invoices.* FROM billing_invoices
		INNER JOIN billing_customers ON billing_invoices.customer_id = billing_customers.id
		WHERE billing_customers.group_id = $1 ORDER BY created_at DESC`
	if tx == nil {
		err = db.DB.Select(&ret, query, groupID)
	} else {
		err = tx.Select(&ret, query, groupID)
	}
	if err != nil {
		logger.Error("finding invoices", rz.Err(err),
			rz.String("group.id", groupID.String()))
		return ret, NewError(ErrorInvoiceNotFound)
	}

	return ret, nil
}
