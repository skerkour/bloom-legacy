package billing

import (
	"context"
	"strings"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func UpdatePlan(ctx context.Context, tx *sqlx.Tx, user *users.User, plan *Plan, name, stripeId, description, tier string, price float64, isActive bool) (*Plan, error) {
	var err error
	logger := rz.FromCtx(ctx)

	if !user.IsAdmin {
		return plan, NewError(ErrorAdminRolRequired)
	}

	// clean and validate params
	name = strings.TrimSpace(name)
	stripeId = strings.TrimSpace(stripeId)
	description = strings.TrimSpace(description)
	tier = strings.TrimSpace(tier)
	if err = validateCreatePlan(name, description, tier, stripeId, price); err != nil {
		return plan, err
	}

	plan.UpdatedAt = time.Now().UTC()
	plan.Name = name
	plan.Description = description
	plan.Tier = tier
	plan.Price = price
	plan.IsActive = isActive
	plan.StripeID = stripeId

	// create group
	queryUpdatePlan := `UPDATE billing_plans
		(updated_at, name, description, stripe_id, price, is_active, tier)
		VALUES ($1, $2, $3, $4, $5, $6, $7) WHERE id = $8`
	_, err = tx.Exec(queryUpdatePlan, plan.UpdatedAt, plan.Name, plan.Description,
		plan.StripeID, plan.Price, plan.IsActive, plan.Tier, plan.ID)
	if err != nil {
		logger.Error("billing.CreatePlan: inserting new plan", rz.Err(err))
		return plan, NewError(ErrorUpdatingPlan)
	}

	return plan, err
}
