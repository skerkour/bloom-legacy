package billing

import (
	"context"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"github.com/stripe/stripe-go/plan"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func CreatePlan(ctx context.Context, tx *sqlx.Tx, user *users.User, name, stripeId, description, product string, storage int64, isPublic bool) (*Plan, error) {
	var ret *Plan
	var err error
	logger := rz.FromCtx(ctx)

	// clean and validate params
	if user == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return ret, NewError(ErrorCreatingPlan)
	}
	if !user.IsAdmin {
		return ret, NewError(ErrorAdminRoleRequired)
	}

	name = strings.TrimSpace(name)
	stripeId = strings.TrimSpace(stripeId)
	description = strings.TrimSpace(description)
	product = strings.TrimSpace(product)
	if err = validateCreatePlan(name, description, product, stripeId, storage); err != nil {
		return ret, err
	}

	stripePlan, err := plan.Get(stripeId, nil)
	if err != nil {
		return ret, NewError(ErrorPlanNotFound)
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
		Price:       stripePlan.Amount,
		IsPublic:    isPublic,
		Product:     product,
		Storage:     storage,
	}

	// create plan
	queryCreatePlan := `INSERT INTO billing_plans
		(id, created_at, updated_at, name, description, stripe_id, price, is_public, product, storage)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`
	_, err = tx.Exec(queryCreatePlan, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Name, ret.Description,
		ret.StripeID, ret.Price, ret.IsPublic, ret.Product, ret.Storage)
	if err != nil {
		logger.Error("billing.CreatePlan: inserting new plan", rz.Err(err))
		return ret, NewError(ErrorCreatingPlan)
	}

	return ret, err
}

func validateCreatePlan(name, description, product, stripeId string, storage int64) error {
	var err error

	if err = validator.BillingPlanName(name); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	if err = validator.BillingProduct(product); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	// if err = validator.BillingPlanPrice(price); err != nil {
	// 	return NewErrorMessage(ErrorInvalidArgument, err.Error())
	// }

	if err = validator.BillingPlanStripeId(stripeId); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	if storage < 0 {
		return NewError(ErrorPlanStorageCantBeNegative)
	}

	return nil
}
