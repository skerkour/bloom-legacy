package billing

import (
	"context"
	"strings"
	"time"

	"github.com/jmoiron/sqlx"
	stripeplan "github.com/stripe/stripe-go/plan"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func UpdatePlan(ctx context.Context, tx *sqlx.Tx, user *users.User, plan *Plan, name, stripeId, description, product string, isPublic bool, storage int64) (*Plan, error) {
	var err error
	logger := rz.FromCtx(ctx)

	// clean and validate params
	if user == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return plan, NewError(ErrorUpdatingPlan)
	}
	if !user.IsAdmin {
		return plan, NewError(ErrorAdminRoleRequired)
	}

	name = strings.TrimSpace(name)
	stripeId = strings.TrimSpace(stripeId)
	description = strings.TrimSpace(description)
	product = strings.TrimSpace(product)
	if err = validateCreatePlan(name, description, product, stripeId, storage); err != nil {
		return plan, err
	}

	stripePlan, err := stripeplan.Get(stripeId, nil)
	if err != nil {
		return plan, NewError(ErrorPlanNotFound)
	}

	plan.UpdatedAt = time.Now().UTC()
	plan.Name = name
	plan.Description = description
	plan.Product = product
	plan.Price = stripePlan.Amount
	plan.IsPublic = isPublic
	plan.StripeID = stripeId
	plan.Storage = storage

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

	return plan, err
}
