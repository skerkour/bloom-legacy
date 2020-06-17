package service

import (
	"context"
	"strings"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) CreatePlan(ctx context.Context, params billing.CreatePlanParams) (ret billing.Plan, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	if !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	params.Name = strings.TrimSpace(params.Name)
	params.StripeID = strings.TrimSpace(params.StripeID)
	params.Description = strings.TrimSpace(params.Description)
	params.Product = strings.TrimSpace(params.Product)
	err = validateCreatePlan(params.Name, params.Description, params.Product, params.StripeID, params.Storage)
	if err != nil {
		return
	}

	now := time.Now().UTC()
	ret = Plan{
		ID:          uuid.New(),
		CreatedAt:   now,
		UpdatedAt:   now,
		Name:        params.Name,
		Description: params.Description,
		StripeID:    params.StripeID,
		Price:       stripePlan.Amount,
		IsPublic:    params.IsPublic,
		Product:     params.Product,
		Storage:     params.Storage,
	}
	err = service.billingRepo.CreatePlan(ctx, service.db, ret)
	return
}

func validateCreatePlan(name, description, product, stripeID string, storage int64) (err error) {
	err = validatePlanName(name)
	if err != nil {
		return
	}

	err = validateProduct(product)
	if err != nil {
		return
	}

	// if err = validator.BillingPlanPrice(price); err != nil {
	// 	return NewErrorMessage(ErrorInvalidArgument, err.Error())
	// }

	err = validatePlanStripeID(stripeID)
	if err != nil {
		return
	}

	if storage < 0 {
		return billing.ErrPlanStorageCantBeNegative
	}

	return nil
}
