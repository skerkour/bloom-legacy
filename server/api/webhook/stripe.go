package webhook

import (
	"encoding/json"
	"io/ioutil"
	"net/http"

	"github.com/stripe/stripe-go"
	"github.com/stripe/stripe-go/webhook"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/libs/rz-go"
)

func StripeHandler(w http.ResponseWriter, req *http.Request) {
	ctx := req.Context()
	logger := rz.FromCtx(ctx)
	var err error
	const MaxBodyBytes = int64(256000)

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
		_, err = billing.CreateOrUpdateInvoice(ctx, &invoice)
	case "invoice.payment_succeeded":
		var invoice stripe.Invoice
		err := json.Unmarshal(event.Data.Raw, &invoice)
		if err != nil {
			logger.Error("Error parsing webhook JSON", rz.Err(err))
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		_, err = billing.CreateOrUpdateInvoice(ctx, &invoice)
	case "invoice.payment_failed":
		var invoice stripe.Invoice
		err := json.Unmarshal(event.Data.Raw, &invoice)
		if err != nil {
			logger.Error("Error parsing webhook JSON", rz.Err(err))
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		_ = billing.PaymentFailed(ctx, &invoice)
	default:
		logger.Info("No action for stripe event", rz.String("type", event.Type))
		w.WriteHeader(http.StatusOK)
		return
	}

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
