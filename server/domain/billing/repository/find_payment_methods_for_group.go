package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindPaymentMethodsForGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret []billing.PaymentMethod, err error) {
	ret = []billing.PaymentMethod{}
	query := `SELECT billing_payment_methods.* FROM billing_payment_methods
		INNER JOIN billing_customers ON billing_payment_methods.customer_id = billing_customers.id
		WHERE billing_customers.group_id = $1`

	err = db.Select(ctx, &ret, query, groupID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.FindPaymentMethodsForGroup: finding payment methods"
		logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", groupID))
		err = errors.Internal(errMessage, err)
	}
	return
}
