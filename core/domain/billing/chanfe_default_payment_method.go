package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/lily/graphql"
)

func ChangeDefaultPaymentMethod(input model.ChangeDefaultPaymentMethodInput) (*model.PaymentMethod, error) {
	client := api.Client()

	var resp struct {
		Method *model.PaymentMethod `json:"changeDefaultPaymentMethod"`
	}
	req := graphql.NewRequest(`
	mutation($input: ChangeDefaultPaymentMethodInput!) {
		changeDefaultPaymentMethod(input: $input) {
			id
			createdAt
			cardLast4
			cardExpirationMonth
			cardExpirationYear
			isDefault
		}
	}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.Method, err
}
