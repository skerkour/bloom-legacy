package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
)

func FindCustomerByStripeCustomerId(ctx context.Context, tx *sqlx.Tx, stripeCustomerId string) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE stripe_customer_id = $1"
	if tx == nil {
		err = tx.Get(ret, queryFind, stripeCustomerId)
	} else {
		err = db.DB.Get(ret, queryFind, stripeCustomerId)
	}
	if err != nil {
		logger.Error("billing.FindCustomerByStripeCustomerId: finding customer", rz.Err(err),
			rz.String("srtripe_customer_id", stripeCustomerId))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}
