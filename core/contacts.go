package core

import (
	"C"
	"encoding/json"
	"gitlab.com/bloom42/bloom/core/domain/contacts"
)

func handleContactsMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "list_contacts":
		res, err := contacts.ListContacts()
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
	case "delete_contact":
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
	case "update_contact":
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
