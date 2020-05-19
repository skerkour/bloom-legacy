package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindCustomerByGroupID(ctx context.Context, tx *sqlx.Tx, groupId uuid.UUID, forUpdate bool) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE group_id = $1"
	if forUpdate {
		queryFind += " FOR UPDATE"
	}
	if tx == nil {
		err = db.DB.Get(ret, queryFind, groupId)
	} else {
		err = tx.Get(ret, queryFind, groupId)
	}
	if err != nil {
		logger.Error("billing.FindCustomerByGroupID: finding customer", rz.Err(err),
			rz.String("group.id", groupId.String()))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}
