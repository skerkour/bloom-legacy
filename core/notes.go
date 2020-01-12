package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/bloom/notes"
)

func HandleNotesMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "list_notes":
		var params notes.ListNotesParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return MethodNotFoundError(method, "notes") // TODO(z0mbie42): return error
		}
		res, err := notes.ListNotes(params)
		if err != nil {
			return MethodNotFoundError(method, "notes") // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return MethodNotFoundError(method, "notes")
	}
}
