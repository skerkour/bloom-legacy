package core

import (
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/db"
)

func HandleCoreMethod(method string, _ json.RawMessage) MessageOut {
	switch method {
	case "init":
		err := db.Init()
		if err != nil {
			return MethodNotFoundError(method, "core") // TODO(z0mbie42): return error
		}
		return MessageOut{Data: InitRes{Success: true}}
	default:
		return MethodNotFoundError(method, "core")
	}
}
