package billing

import (
	"context"
	"strings"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func UpdatePlan(ctx context.Context, tx *sqlx.Tx, user *users.User, plan *Plan, name, stripeId, description, tier string, price float64, isActive bool, storage int64) (*Plan, error) {
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
	tier = strings.TrimSpace(tier)
	if err = validateCreatePlan(name, description, tier, stripeId, price, storage); err != nil {
		return plan, err
	}

	plan.UpdatedAt = time.Now().UTC()
	plan.Name = name
	plan.Description = description
	plan.Tier = tier
	plan.Price = price
	plan.IsActive = isActive
	plan.StripeID = stripeId
	plan.Storage = storage

	// create group
	queryUpdatePlan := `UPDATE billing_plans SET updated_at = $1, name = $2, description = $3,
		stripe_id = $4, price = $5, is_active = $6, tier = $7, storage = $8
		WHERE id = $9`
	_, err = tx.Exec(queryUpdatePlan, plan.UpdatedAt, plan.Name, plan.Description,
		plan.StripeID, plan.Price, plan.IsActive, plan.Tier, plan.Storage, plan.ID)
	if err != nil {
		logger.Error("billing.CreatePlan: inserting new plan", rz.Err(err))
		return plan, NewError(ErrorUpdatingPlan)
	}

	return plan, err
}
