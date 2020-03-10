package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/contacts"
)

func handleContactsMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "listContacts":
		res, err := contacts.ListContacts()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "createContact":
		var params contacts.CreateContactParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := contacts.CreateContact(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "deleteContact":
		var params contacts.DeleteContactParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := contacts.DeleteContact(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "updateContact":
		var params contacts.Contact
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := contacts.UpdateContact(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return methodNotFoundError(method, "contacts")
	}
}
