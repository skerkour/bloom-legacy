package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
)

func CustomerUpdateUsedStorage(ctx context.Context, tx *sqlx.Tx, customer *Customer, amount int64) (err error) {
	logger := rz.FromCtx(ctx)

	diff := customer.UsedStorage + amount
	if diff < 0 {
		err = NewError(ErrorInvalidCustomerStorage)
		return
	}

	customer.UpdatedAt = time.Now().UTC()
	customer.UsedStorage = diff

	query := "UPDATE billing_customers SET updated_at = $1, used_storage = $2 WHERE id = $3"
	if tx == nil {
		_, err = db.DB.Exec(query, customer.UpdatedAt, customer.UsedStorage, customer.ID)
	} else {
		_, err = tx.Exec(query, customer.UpdatedAt, customer.UsedStorage, customer.ID)
	}
	if err != nil {
		logger.Error("billing.CustomerUpdateUsedStorage: updating customer", rz.Err(err),
			rz.String("customer.id", customer.ID.String()))
		err = NewError(ErrorInternal)
		return
	}

	return
}
