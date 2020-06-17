package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindCustomerByUserId(ctx context.Context, tx *sqlx.Tx, userId uuid.UUID, forUpdate bool) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE user_id = $1"
	if forUpdate {
		queryFind += " FOR UPDATE"
	}
	if tx == nil {
		err = db.DB.Get(ret, queryFind, userId)
	} else {
		err = tx.Get(ret, queryFind, userId)
	}
	if err != nil {
		logger.Error("billing.FindCustomerByUserId: finding customer", rz.Err(err),
			rz.String("user.id", userId.String()))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}
