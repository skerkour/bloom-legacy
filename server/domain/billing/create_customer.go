package billing

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/rz-go"
)

// creates a customer only for us
func CreateCustomer(ctx context.Context, tx *sqlx.Tx, userID, groupID *string) (Customer, twirp.Error) {
	logger := rz.FromCtx(ctx)
	var err error
	var ret Customer

	// fetch basic plan
	defaultPlanID := ""

	// create customer
	now := time.Now().UTC()
	newUuid := uuid.New()
	ret = Customer{
		ID:        newUuid.String(),
		CreatedAt: now,
		UpdatedAt: now,
		PlanID:    defaultPlanID,
		StripeID:  nil,
		UserID:    userID,
		GroupID:   groupID,
	}

	queryCreateCustomer := `INSERT INTO billing_customers
		(id, created_at, updated_at, plan_id, stripe_id, user_id, group_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err = tx.Exec(queryCreateCustomer, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.PlanID,
		ret.StripeID, ret.UserID, ret.GroupID)
	if err != nil {
		logger.Error("billing.CreateCustomer: inserting new customer", rz.Err(err))
		return ret, twirp.InternalError(ErrorCreatingCustomerMsg)
	}
	return ret, nil
}
