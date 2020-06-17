package service

import (
	"context"
	"strings"
	"time"

	stripeplan "github.com/stripe/stripe-go/plan"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (service *BillingService) UpdatePlan(ctx context.Context, params billing.UpdatePlanParams) (plan billing.Plan, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	if !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}
	logger := log.FromCtx(ctx)

	params.Name = strings.TrimSpace(params.Name)
	params.StripeID = strings.TrimSpace(params.StripeID)
	params.Description = strings.TrimSpace(params.Description)
	params.Product = strings.TrimSpace(params.Product)
	err = validateCreatePlan(params.Name, params.Description, params.Product, params.StripeID, params.Storage)
	if err != nil {
		return
	}

	stripePlan, err := stripeplan.Get(params.StripeID, nil)
	if err != nil {
		errMessage := "billing.UpdatePlan: fetching stripe plan"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "billing.UpdatePlan: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	plan, err = service.billingRepo.FindPlanByID(ctx, tx, params.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	plan.UpdatedAt = time.Now().UTC()
	plan.Name = params.Name
	plan.Description = params.Description
	plan.Product = params.Product
	plan.Price = stripePlan.Amount
	plan.StripeID = params.StripeID
	plan.Storage = params.Storage

	err = service.billingRepo.UpdatePlan(ctx, tx, plan)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "billing.UpdatePlan: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
