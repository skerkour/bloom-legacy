package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) FindInvoicesForGroup(ctx context.Context, groupID uuid.UUID) (ret []billing.Invoice, err error) {
	return
}

/*
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdmin(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	invoices, err := billing.FindInvoicesByGroupId(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}
*/
