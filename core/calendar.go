package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/calendar"
	"gitlab.com/bloom42/bloom/core/messages"
)
import "gitlab.com/bloom42/bloom/core/domain/objects"

func handleCalendarMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "findEvents":
		var params messages.CalendarFindEventsParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := calendar.FindEvents(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "createEvent":
		var params messages.CalendarCreateEventParams
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
		var params messages.CalendarDeleteEventParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		err = calendar.DeleteEvent(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: messages.Empty{}}
	case "updateEvent":
		var params objects.Object
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
