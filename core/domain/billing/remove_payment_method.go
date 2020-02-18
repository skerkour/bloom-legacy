package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func RemovePaymentMethod(input model.RemovePaymentMethodInput) error {
	client := api.Client()

	resp := map[string]interface{}{}
	req := graphql.NewRequest(`
		mutation ($input: RemovePaymentMethodInput!) {
			removePaymentMethod(input: $input)
		}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return err
}
