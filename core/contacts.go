package core

import (
	"C"
	"encoding/json"
	"gitlab.com/bloom42/bloom/core/bloom/contacts"
	"gitlab.com/bloom42/bloom/core/bloom/kernel"
)

func handleContactsMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "list_contacts":
		var params kernel.Empty
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := contacts.ListContacts(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "create_contact":
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
	default:
		return methodNotFoundError(method, "contacts")
	}
}
