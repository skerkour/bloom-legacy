package webhook

import (
	"encoding/json"
	"io/ioutil"
	"net/http"

	"github.com/stripe/stripe-go"
	"github.com/stripe/stripe-go/webhook"
	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/log"
)

type WebhookAPI struct {
	billingService billing.Service
}

// NewAPI returns a new webhook API with the appropriate dependencies
func NewAPI(billingService billing.Service) WebhookAPI {
	return WebhookAPI{
		billingService: billingService,
	}
}

func (api *WebhookAPI) StripeHandler(w http.ResponseWriter, req *http.Request) {
	ctx := req.Context()
	logger := log.FromCtx(ctx)
	var err error
	const MaxBodyBytes = int64(256000)

	req.Body = http.MaxBytesReader(w, req.Body, MaxBodyBytes)
	payload, err := ioutil.ReadAll(req.Body)
	if err != nil {
		logger.Error("webhoo.StripeHandler: Error reading request body", log.Err("error", err))
		w.WriteHeader(http.StatusServiceUnavailable)
		return
	}

	signature := req.Header.Get("Stripe-Signature")
	event, err := webhook.ConstructEvent(payload, signature, config.Stripe.WebhookSecret)
	if err != nil {
		logger.Error("webhook.StripeHandler: Failed to verify webhook secret", log.Err("error", err))
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	// Unmarshal the event data into an appropriate struct depending on its Type
	switch event.Type {
	case "invoice.created":
		var invoice stripe.Invoice
		err := json.Unmarshal(event.Data.Raw, &invoice)
		if err != nil {
			logger.Error("Error parsing webhook JSON", log.Err("error", err))
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		err = api.billingService.CreateOrUpdateInvoice(ctx, invoice)
	case "invoice.payment_succeeded":
		var invoice stripe.Invoice
		err := json.Unmarshal(event.Data.Raw, &invoice)
		if err != nil {
			logger.Error("Error parsing webhook JSON", log.Err("error", err))
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		err = api.billingService.CreateOrUpdateInvoice(ctx, invoice)
	case "invoice.payment_failed":
		var invoice stripe.Invoice
		err := json.Unmarshal(event.Data.Raw, &invoice)
		if err != nil {
			logger.Error("Error parsing webhook JSON", log.Err("error", err))
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		err = api.billingService.PaymentFailed(ctx, &invoice)
	default:
		logger.Info("No action for stripe event", log.String("type", event.Type))
		w.WriteHeader(http.StatusOK)
		return
	}

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
