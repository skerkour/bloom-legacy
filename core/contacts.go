package core


import (
	"C"
	"encoding/json"
)

func handleContactsMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	default:
		return methodNotFoundError(method, "contacts")
	}
}
