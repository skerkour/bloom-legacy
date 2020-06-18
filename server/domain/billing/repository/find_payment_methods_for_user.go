package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindPaymentMethodsForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret []billing.PaymentMethod, err error) {
	ret = []billing.PaymentMethod{}
	query := `SELECT billing_payment_methods.* FROM billing_payment_methods
		INNER JOIN billing_customers ON billing_payment_methods.customer_id = billing_customers.id
		WHERE billing_customers.user_id = $1`

	err = db.Select(ctx, &ret, query, userID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.FindPaymentMethodsForUser: finding payment methods"
		logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", userID))
		err = errors.Internal(errMessage, err)
	}
	return
}
