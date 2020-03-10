package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/calendar"
)

func handleCalendarMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "listEvents":
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
	case "createEvent":
		var params calendar.CreateEventParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := calendar.CreateEvent(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "deleteEvent":
		var params calendar.DeleteEventParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := calendar.DeleteEvent(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "updateEvent":
		var params calendar.Event
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := calendar.UpdateEvent(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return methodNotFoundError(method, "calendar")
	}
}
