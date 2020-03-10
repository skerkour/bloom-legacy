package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/notes"
)

func handleNotesMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "listNotes":
		res, err := notes.ListNotes()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "listArchived":
		res, err := notes.ListArchived()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "createNote":
		var params notes.CreateNoteParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := notes.CreateNote(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "updateNote":
		var params notes.Note
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := notes.UpdateNote(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "deleteNote":
		var params notes.DeleteNoteParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := notes.DeleteNote(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return methodNotFoundError(method, "notes")
	}
}
