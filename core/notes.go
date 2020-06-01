package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/notes"
	"gitlab.com/bloom42/bloom/core/messages"
)
import "gitlab.com/bloom42/bloom/core/domain/objects"

func handleNotesMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "findNotes":
		var params messages.NotesFindParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := notes.FindNotes(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "findArchived":
		var params messages.NotesFindParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := notes.FindArchived(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "createNote":
		var params messages.CreateNoteParams
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
		var params objects.Object
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
		var params messages.DeleteNoteParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		err = notes.DeleteNote(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: messages.Empty{}}
	default:
		return methodNotFoundError(method, "notes")
	}
}
