package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/billing"
)

func handleBillingMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "fetch_my_profile":
		res, err := billing.FetchMyProfile()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "fetch_plans":
		res, err := billing.FetchPlans()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return methodNotFoundError(method, "billing")
	}
}
