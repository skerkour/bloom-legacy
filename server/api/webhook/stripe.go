package webhook

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"

	"github.com/stripe/stripe-go"
	"github.com/stripe/stripe-go/webhook"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/libs/rz-go"
)

// TODO: check signature
func StripeHandler(w http.ResponseWriter, req *http.Request) {
	logger := rz.FromCtx(req.Context())
	const MaxBodyBytes = int64(65536)
	req.Body = http.MaxBytesReader(w, req.Body, MaxBodyBytes)
	payload, err := ioutil.ReadAll(req.Body)
	if err != nil {
		logger.Error("Error reading request body", rz.Err(err))
		w.WriteHeader(http.StatusServiceUnavailable)
		return
	}

	event, err := webhook.ConstructEvent(payload, req.Header.Get("Stripe-Signature"), config.Stripe.WebhookSecret)
	if err != nil {
		logger.Error("Failed to verify webhook secret", rz.Err(err))
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	fmt.Printf("STRIPE EVENT: %s\n", string(payload))

	// Unmarshal the event data into an appropriate struct depending on its Type
	switch event.Type {
	case "payment_intent.succeeded":
		var paymentIntent stripe.PaymentIntent
		err := json.Unmarshal(event.Data.Raw, &paymentIntent)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error parsing webhook JSON: %v\\n", err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		// Then define and call a func to handle the successful payment intent.
		// handlePaymentIntentSucceeded(paymentIntent)
	case "payment_method.attached":
		var paymentMethod stripe.PaymentMethod
		err := json.Unmarshal(event.Data.Raw, &paymentMethod)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error parsing webhook JSON: %v\\n", err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		// Then define and call a func to handle the successful attachment of a PaymentMethod.
		// handlePaymentMethodAttached(paymentMethod)
	// ... handle other event types
	default:
		fmt.Fprintf(os.Stderr, "Unexpected event type: %s\\n", event.Type)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}
