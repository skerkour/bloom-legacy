package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/libs/rz-go"
)

func FindCustomerByUserId(ctx context.Context, tx *sqlx.Tx, userId string) (*Customer, error) {
	var ret *Customer
	var customer Customer
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE user_id = $1"
	err = tx.Get(&customer, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindCustomerByUserId: finding customer", rz.Err(err),
			rz.String("id", userId))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &customer
	return ret, err
}

func FindCustomerByGroupId(ctx context.Context, tx *sqlx.Tx, groupId string) (*Customer, error) {
	var ret *Customer
	var customer Customer
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE group_id = $1"
	err = tx.Get(&customer, queryFind, groupId)
	if err != nil {
		logger.Error("billing.FindCustomerByGroupId: finding customer", rz.Err(err),
			rz.String("id", groupId))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &customer
	return ret, err
}
