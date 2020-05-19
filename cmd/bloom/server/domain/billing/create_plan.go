package billing

import (
	"context"
	"strings"
	"time"

	"github.com/stripe/stripe-go/plan"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

type CreatePlanParams struct {
	Name        string
	StripeID    string
	Description string
	Product     string
	Storage     int64
	IsPublic    bool
}

func CreatePlan(ctx context.Context, actor *users.User, params CreatePlanParams) (ret *Plan, err error) {
	logger := rz.FromCtx(ctx)

	// clean and validate params
	if actor == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return ret, NewError(ErrorCreatingPlan)
	}
	if !actor.IsAdmin {
		return ret, NewError(ErrorAdminRoleRequired)
	}

	params.Name = strings.TrimSpace(params.Name)
	params.StripeID = strings.TrimSpace(params.StripeID)
	params.Description = strings.TrimSpace(params.Description)
	params.Product = strings.TrimSpace(params.Product)
	err = validateCreatePlan(params.Name, params.Description, params.Product, params.StripeID, params.Storage)
	if err != nil {
		return
	}

	stripePlan, err := plan.Get(params.StripeID, nil)
	if err != nil {
		return ret, NewError(ErrorPlanNotFound)
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("billing.CreatePlan: Starting transaction", rz.Err(err))
		err = NewError(ErrorCreatingPlan)
		return
	}

	now := time.Now().UTC()
	newUuid := uuid.New()

	ret = &Plan{
		ID:          newUuid,
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

	// create plan
	queryCreatePlan := `INSERT INTO billing_plans
		(id, created_at, updated_at, name, description, stripe_id, price, is_public, product, storage)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`
	_, err = tx.Exec(queryCreatePlan, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Name, ret.Description,
		ret.StripeID, ret.Price, ret.IsPublic, ret.Product, ret.Storage)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.CreatePlan: inserting new plan", rz.Err(err))
		err = NewError(ErrorCreatingPlan)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("billing.CreatePlan: Committing transaction", rz.Err(err))
		err = NewError(ErrorCreatingPlan)
		return
	}

	return ret, err
}

func validateCreatePlan(name, description, product, stripeID string, storage int64) (err error) {

	err = ValidatePlanName(name)
	if err != nil {
		err = NewErrorMessage(ErrorInvalidArgument, err.Error())
		return
	}

	err = ValidateProduct(product)
	if err != nil {
		err = NewErrorMessage(ErrorInvalidArgument, err.Error())
		return
	}

	// if err = validator.BillingPlanPrice(price); err != nil {
	// 	return NewErrorMessage(ErrorInvalidArgument, err.Error())
	// }

	err = ValidatePlanStripeID(stripeID)
	if err != nil {
		err = NewErrorMessage(ErrorInvalidArgument, err.Error())
		return
	}

	if storage < 0 {
		return NewError(ErrorPlanStorageCantBeNegative)
	}

	return nil
}
