package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

type Plan struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	Name        string `json:"name" db:"name"`
	StripeID    string `json:"stripe_id" db:"stripe_id"`
	Price       int64  `json:"price" db:"price"`
	Description string `json:"description" db:"description"`
	IsPublic    bool   `json:"is_public" db:"is_public"`
	Product     string `json:"product" db:"product"`
	Storage     int64  `json:"storage" db:"storage"`
}

func FindDefaultPlan(ctx context.Context, tx *sqlx.Tx) (*Plan, error) {
	var ret *Plan
	var plan Plan
	var err error
	logger := rz.FromCtx(ctx)

	// queryFindPlan := "SELECT * FROM billing_plans WHERE product = $1 AND is_public = $2"
	// err = tx.Get(&plan, queryFindPlan, consts.BILLING_PRODUCT_FREE, true)
	queryFindPlan := "SELECT * FROM billing_plans WHERE id = $1"
	err = tx.Get(&plan, queryFindPlan, DefaultBillingPlanId)
	if err != nil {
		logger.Error("billing.FindDefaultPlan: finding plan by id", rz.Err(err))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &plan
	return ret, err
}

func FindPublicPlanById(ctx context.Context, tx *sqlx.Tx, planId string) (*Plan, error) {
	var ret *Plan
	var plan Plan
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT * FROM billing_plans WHERE id = $1 AND is_public = true"
	err = tx.Get(&plan, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.FindPublicPlanById: finding plan by id", rz.Err(err),
			rz.String("id", planId))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &plan
	return ret, err
}

func FindPlanById(ctx context.Context, tx *sqlx.Tx, planId string) (*Plan, error) {
	var ret *Plan
	var plan Plan
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT * FROM billing_plans WHERE id = $1"
	err = tx.Get(&plan, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.FindPlanById: finding plan by id", rz.Err(err),
			rz.String("id", planId))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &plan
	return ret, err
}

func GetSubscribersCountForPlanId(ctx context.Context, planId string) (int64, error) {
	var ret int64
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT COUNT(*) FROM billing_customers WHERE plan_id = $1"
	err = db.DB.Get(&ret, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.GetSubscribersCountForPlanId: finding plan by id", rz.Err(err),
			rz.String("id", planId))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}

func GetSubscribersCountForPlanIdTx(ctx context.Context, tx *sqlx.Tx, planId string) (int64, error) {
	var ret int64
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT COUNT(*) FROM billing_customers WHERE plan_id = $1"
	err = tx.Get(&ret, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.GetSubscribersCountForPlanIdTx: finding plan by id", rz.Err(err),
			rz.String("id", planId))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}

func FindPlans(ctx context.Context, user *users.User) ([]Plan, error) {
	if user == nil || !user.IsAdmin {
		return FindPlansForUser(ctx)
	} else {
		return FindPlansForAdmin(ctx)
	}
}

func FindPlansForUser(ctx context.Context) ([]Plan, error) {
	ret := []Plan{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_plans WHERE is_public = true"
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

	queryFind := "SELECT * FROM billing_plans"
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

	queryFind := `SELECT billing_plans.* FROM billing_plans
	INNER JOIN billing_customers ON billing_plans.id = billing_customers.plan_id
	WHERE billing_customers.user_id = $1`
	err = db.DB.Get(ret, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindPlanForUser: finding plan", rz.Err(err))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}

func FindPlanForCustomer(ctx context.Context, customer *Customer) (*Plan, error) {
	ret := &Plan{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT billing_plans.* FROM billing_plans WHERE id = $1"
	err = db.DB.Get(ret, queryFind, customer.PlanID)
	if err != nil {
		logger.Error("billing.FindPlanForCustomer: finding plan", rz.Err(err))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}
