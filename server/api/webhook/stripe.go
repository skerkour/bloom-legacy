package webhook

import (
	"encoding/json"
	"io/ioutil"
	"net/http"

	"github.com/stripe/stripe-go"
	"github.com/stripe/stripe-go/webhook"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/libs/rz-go"
)

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

	signature := req.Header.Get("Stripe-Signature")
	event, err := webhook.ConstructEvent(payload, signature, config.Stripe.WebhookSecret)
	if err != nil {
		logger.Error("Failed to verify webhook secret", rz.Err(err))
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	// Unmarshal the event data into an appropriate struct depending on its Type
	switch event.Type {
	case "invoice.created":
		var invoice stripe.Invoice
		err := json.Unmarshal(event.Data.Raw, &invoice)
		if err != nil {
			logger.Error("Error parsing webhook JSON", rz.Err(err))
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		// TODO: create invoice
	case "invoice.payment_succeeded":
		var invoice stripe.Invoice
		err := json.Unmarshal(event.Data.Raw, &invoice)
		if err != nil {
			logger.Error("Error parsing webhook JSON", rz.Err(err))
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		// TODO: update invoice
	case "invoice.payment_action_required":
		var invoice stripe.Invoice
		err := json.Unmarshal(event.Data.Raw, &invoice)
		if err != nil {
			logger.Error("Error parsing webhook JSON", rz.Err(err))
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		// TODO: send email
	default:
		logger.Info("No action for stripe event", rz.String("type", event.Type))
		w.WriteHeader(http.StatusOK)
		return
	}

	w.WriteHeader(http.StatusOK)
}
