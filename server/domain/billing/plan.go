package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

type Plan struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	Name        string  `json:"name" db:"name"`
	StripeID    string  `json:"stripe_id" db:"stripe_id"`
	Price       float64 `json:"price" db:"price"`
	Description string  `json:"description" db:"description"`
	IsActive    bool    `json:"is_active" db:"is_active"`
	Tier        string  `json:"tier" db:"tier"`
	Storage     int64   `json:"storage" db:"storage"`
}

func FindDefaultPlan(ctx context.Context, tx *sqlx.Tx) (*Plan, error) {
	var ret *Plan
	var plan Plan
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT * FROM billing_plans WHERE tier = $1 AND is_active = $2"
	err = tx.Get(&plan, queryFindPlan, consts.BILLING_FREE_TIER, true)
	if err != nil {
		logger.Error("billing.FindDefaultPlan: finding plan by id", rz.Err(err))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &plan
	return ret, err
}

func FindPlanActiveById(ctx context.Context, tx *sqlx.Tx, planId string) (*Plan, error) {
	var ret *Plan
	var plan Plan
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT * FROM billing_plans WHERE id = $1 AND is_active = true"
	err = tx.Get(&plan, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.FindPlanActiveById: finding plan by id", rz.Err(err),
			rz.String("id", planId))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &plan
	return ret, err
}

func FindPlansForUser(ctx context.Context) ([]Plan, error) {
	ret := []Plan{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_plans WHERE is_active = true"
	err = db.DB.Select(&ret, queryFind)
	if err != nil {
		logger.Error("billing.FindPlansForUser: finding plans", rz.Err(err))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}

func FindPlansForAdmin(ctx context.Context) ([]Plan, error) {
	ret := []Plan{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_plans WHERE"
	err = db.DB.Select(&ret, queryFind)
	if err != nil {
		logger.Error("billing.FindPlansForAdmin: finding plans", rz.Err(err))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}

func FindPlanForUser(ctx context.Context, userId string) (*Plan, error) {
	ret := &Plan{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := `SELECT * FROM billing_plans, billing_customers
	WHERE billing_plans.id = billing_customers.plan_id AND billing_customers.user_id = $1`
	err = db.DB.Select(ret, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindPlanForUser: finding plan", rz.Err(err))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}
