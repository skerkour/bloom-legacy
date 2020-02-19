package billing

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
	"strings"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func AddPaymentMethod(params AddPaymentMethodParams) (*model.PaymentMethod, error) {
	client := api.Client()

	data := url.Values{}
	data.Set("type", "card")
	data.Set("card[number]", params.Card.Number)
	data.Set("card[exp_month]", params.Card.ExpMonth)
	data.Set("card[exp_year]", params.Card.ExpYear)
	data.Set("card[cvc]", params.Card.Cvc)

	stripeReq, err := http.NewRequest("POST", "https://api.stripe.com/v1/payment_methods", strings.NewReader(data.Encode()))
	if err != nil {
		return nil, err
	}
	stripeReq.SetBasicAuth(*params.StripePublicKey, "")

	httpClient := &http.Client{}
	stripeRes, err := httpClient.Do(stripeReq)
	if err != nil {
		return nil, err
	}
	defer stripeRes.Body.Close()

	body, err := ioutil.ReadAll(stripeRes.Body)
	if err != nil {
		return nil, err
	}
	fmt.Printf("STRIPE RES: %s\n", string(body))

	if stripeRes.StatusCode > 200 {
		var stripeError StripeResError
		err = json.Unmarshal(body, &stripeError)
		if err != nil {
			return nil, err
		}
		return nil, errors.New(stripeError.Error.Message)
	}

	var stripeCard StripeCard
	err = json.Unmarshal(body, &stripeCard)
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
				isDefault
			}
		}
	`)
	req.Var("input", input)

	err = client.Do(context.Background(), req, &resp)

	return resp.AddPaymentMethod, err
}
