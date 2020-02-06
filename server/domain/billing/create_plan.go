package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/common/validator"
)

func CreatePlan(ctx context.Context, tx *sqlx.Tx, name, stripeId, description, tier string, price float64) (*Plan, error) {
	var ret *Plan
	var err error

	// validate params
	if err = validator.BillingPlanName(name); err != nil {
		return ret, NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	if err = validator.BillingPlanTier(tier); err != nil {
		return ret, NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	return ret, err
}
