package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/bloom/calendar"
)

func handleCalendarMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "list_events":
		var params calendar.ListEventsParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := calendar.ListEvents(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return methodNotFoundError(method, "calendar")
	}
}
