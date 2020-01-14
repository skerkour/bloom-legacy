package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/bloom/notes"
	"gitlab.com/bloom42/bloom/core/bloom/kernel"
)

func HandleNotesMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "list_notes":
		var params kernel.Empty
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := notes.ListNotes(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "list_archived":
		var params kernel.Empty
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := notes.ListArchived(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return MethodNotFoundError(method, "notes")
	}
}
