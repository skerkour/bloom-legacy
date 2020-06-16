package billing

import (
	"context"
	"strings"
	"time"

	stripeplan "github.com/stripe/stripe-go/plan"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/rz"
)

func UpdatePlan(ctx context.Context, actor *users.User, params UpdatePlanParams) (ret *Plan, err error) {
	logger := rz.FromCtx(ctx)

	// clean and validate params
	if actor == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		err = NewError(ErrorUpdatingPlan)
		return
	}
	if !actor.IsAdmin {
		err = NewError(ErrorAdminRoleRequired)
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

	stripePlan, err := stripeplan.Get(params.StripeID, nil)
	if err != nil {
		err = NewError(ErrorPlanNotFound)
		return
	}

	if params.ID == nil {
		err = errors.New(errors.InvalidArgument, "plan.id should not be null")
		return
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.UpdateBillingPlan: Starting transaction", rz.Err(err))
		err = NewError(ErrorUpdatingPlan)
		return
	}

	plan, err := FindPlanById(ctx, tx, *params.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	plan.UpdatedAt = time.Now().UTC()
	plan.Name = params.Name
	plan.Description = params.Description
	plan.Product = params.Product
	plan.Price = stripePlan.Amount
	plan.IsPublic = params.IsPublic
	plan.StripeID = params.StripeID
	plan.Storage = params.Storage

	// create group
	queryUpdatePlan := `UPDATE billing_plans SET updated_at = $1, name = $2, description = $3,
		stripe_id = $4, price = $5, is_public = $6, product = $7, storage = $8
		WHERE id = $9`
	_, err = tx.Exec(queryUpdatePlan, plan.UpdatedAt, plan.Name, plan.Description,
		plan.StripeID, plan.Price, plan.IsPublic, plan.Product, plan.Storage, plan.ID)
	if err != nil {
		logger.Error("billing.CreatePlan: inserting new plan", rz.Err(err))
		return plan, NewError(ErrorUpdatingPlan)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.UpdateBillingPlan: Committing transaction", rz.Err(err))
		err = NewError(ErrorCreatingPlan)
		return
	}

	return plan, err
}
