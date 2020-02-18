package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func AddPaymentMethod(input model.AddPaymentMethodInput) (*model.PaymentMethod, error) {
	client := api.Client()

	var resp struct {
		AddPaymentMethod *model.PaymentMethod `json:"addPaymentMethod"`
	}
	req := graphql.NewRequest(`
		mutation ($input: AddPaymentMethodInput!) {
			addPaymentMethod(input: $input) {
				id
				createdAt
				cardLast4
				cardExpirationMonth
				cardExpirationYear
			}
		}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.AddPaymentMethod, err
}
