package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindCustomerByGroupID(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret billing.Customer, err error) {
	query := "SELECT * FROM billing_customers WHERE group_id = $1"

	err = db.Get(ctx, &ret, query, groupID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Customer not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "billing.FindCustomerByGroupID: finding customer"
			logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", groupID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
