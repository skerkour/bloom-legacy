package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) FindInvoicesForUser(ctx context.Context, userID uuid.UUID) (ret []billing.Invoice, err error) {
	return
}

/*

	var ret *InvoiceConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	invoices, err := billing.FindInvoicesByUserId(ctx, nil, uuid.UUID(*user.ID).String())
	if err != nil {
		return ret, gqlerrors.New(err)
	}

*/
