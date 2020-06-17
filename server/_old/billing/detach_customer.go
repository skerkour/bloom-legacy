package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func DetachCustomer(ctx context.Context, tx *sqlx.Tx, userID *uuid.UUID, groupID *uuid.UUID) (err error) {
	logger := rz.FromCtx(ctx)
	var customer *Customer

	if userID == nil && groupID == nil {
		logger.Debug("billing.DetachCustomer: both userID and groupID are null")
		err = NewError(ErrorInternal)
		return
	}

	if userID != nil {
		customer, err = FindCustomerByUserId(ctx, tx, *userID, true)
		if err != nil {
			return
		}
		customer.UserID = nil
	} else {
		customer, err = FindCustomerByGroupID(ctx, tx, *groupID, true)
		if err != nil {
			return
		}
		customer.GroupID = nil
	}

	now := time.Now().UTC()
	customer.UpdatedAt = now

	query := "UPDATE billing_customers SET updated_at = $1, group_id = $2, user_id = $3 WHERE id = $4"
	_, err = tx.Exec(query, customer.UpdatedAt, customer.GroupID, customer.UserID, customer.ID)
	if err != nil {
		logger.Error("billing.DetachCustomer: updating customer", rz.Err(err),
			rz.String("customer.id", customer.ID.String()))
		err = NewError(ErrorInternal)
		return
	}

	return
}
