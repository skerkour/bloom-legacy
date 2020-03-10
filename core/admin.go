package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/admin"
)

func handleAdminMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "fetchDashboardData":
		res, err := admin.FetchDashboardData()
		if err != nil {
			return InternalError(err)
		}
		return MessageOut{Data: res}
	default:
		return methodNotFoundError(method, "admin")
	}
}
