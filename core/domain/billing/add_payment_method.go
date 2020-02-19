package billing

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func AddPaymentMethod(params AddPaymentMethodParams) (*model.PaymentMethod, error) {
	client := api.Client()

	stripePayload := NewStripePaymentMethod{
		Type: "card",
		Card: params.Card,
	}
	stripePayloadJson, err := json.Marshal(stripePayload)
	if err != nil {
		return nil, err
	}

	stripeReq, err := http.NewRequest("POST", "https://api.stripe.com/v1/payment_methods", bytes.NewBuffer(stripePayloadJson))
	if err != nil {
		return nil, err
	}
	stripeReq.Header.Set("Content-Type", "application/json")
	stripeReq.SetBasicAuth(*params.StripePublicKey, "")

	httpClient := &http.Client{}
	res, err := httpClient.Do(stripeReq)
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()

	var stripeCard StripeCard
	err = json.NewDecoder(res.Body).Decode(&stripeCard)
	if err != nil {
		return nil, err
	}

	input := model.AddPaymentMethodInput{
		StripeID: stripeCard.ID,
		GroupID:  params.GroupID,
	}
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

	err = client.Do(context.Background(), req, &resp)

	return resp.AddPaymentMethod, err
}
