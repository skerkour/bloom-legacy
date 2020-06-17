package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (repo *BillingRepository) CreatePlan(ctx context.Context, db db.Queryer, plan billing.Plan) (err error) {
	return
}

/*

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

*/
