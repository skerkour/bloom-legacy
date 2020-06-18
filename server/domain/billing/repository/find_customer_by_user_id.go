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

func (repo *BillingRepository) FindCustomerByUserID(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret billing.Customer, err error) {
	query := "SELECT * FROM billing_customers WHERE user_id = $1"

	err = db.Get(ctx, &ret, query, userID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Customer not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "billing.FindCustomerByUserID: finding customer"
			logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", userID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
