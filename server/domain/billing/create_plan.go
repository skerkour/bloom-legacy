package billing

import (
	"context"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/libs/rz-go"
)

func CreatePlan(ctx context.Context, tx *sqlx.Tx, name, stripeId, description, tier string, price float64) (*Plan, error) {
	var ret *Plan
	var err error
	logger := rz.FromCtx(ctx)

	// clean and validate params
	name = strings.TrimSpace(name)
	stripeId = strings.TrimSpace(stripeId)
	description = strings.TrimSpace(description)
	tier = strings.TrimSpace(tier)
	if err = validateCreatePlan(name, description, tier, stripeId, price); err != nil {
		return ret, err
	}

	now := time.Now().UTC()
	newUuid := uuid.New()

	ret = &Plan{
		ID:          newUuid.String(),
		CreatedAt:   now,
		UpdatedAt:   now,
		Name:        name,
		Description: description,
		StripeID:    stripeId,
		Price:       price,
		IsActive:    false,
		Tier:        tier,
	}

	// create group
	queryCreateGroup := `INSERT INTO billing_plans
		(id, created_at, updated_at, name, description, stripe_id, price, is_active, tier)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`
	_, err = tx.Exec(queryCreateGroup, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Name, ret.Description,
		ret.StripeID, ret.Price, ret.IsActive, ret.Tier)
	if err != nil {
		logger.Error("billing.CreatePlan: inserting new plan", rz.Err(err))
		return ret, NewError(ErrorCreatingPlan)
	}

	return ret, err
}

func validateCreatePlan(name, description, tier, stripeId string, price float64) error {
	var err error

	if err = validator.BillingPlanName(name); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	if err = validator.BillingPlanTier(tier); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	if price < 0.0 {
		NewErrorMessage(ErrorInvalidArgument, "price can't be less than 0")
	}

	if price > 1000000.0 {
		NewErrorMessage(ErrorInvalidArgument, "price can't be more than 1M")
	}

	if !strings.HasPrefix(stripeId, "plan_") {
		NewErrorMessage(ErrorInvalidArgument, "stripe_id does not match the pattern \"plan_*\"")
	}

	return nil
}
