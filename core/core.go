package core

import (
	"encoding/json"
	"errors"
	"strings"

	"gitlab.com/bloom42/bloom/core/db"
)

func handleCoreMethod(method string, _ json.RawMessage) MessageOut {
	switch method {
	case "init":
		err := db.Init()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: InitRes{Success: true}}
	default:
		return methodNotFoundError(method, "core")
	}
}

func HandleMessage(messageIn MessageIn) ([]byte, error) {
	parts := strings.Split(messageIn.Method, ".")
	if len(parts) != 2 {
		return nil, errors.New("method is malformed") // TODO(z0mbie42): return error
	}

	// TODO(z0mbie42): handle methods returns go error, and convert to c error here
	var messageOut MessageOut

	switch parts[0] {
	case "users":
		messageOut = handleUsersMethod(parts[1], messageIn.Params)
	case "calculator":
		messageOut = handleCalculatorMehtod(parts[1], messageIn.Params)
	case "calendar":
		messageOut = handleCalendarMehtod(parts[1], messageIn.Params)
	case "contacts":
		messageOut = handleContactsMehtod(parts[1], messageIn.Params)
	case "core":
		messageOut = handleCoreMethod(parts[1], messageIn.Params)
	case "notes":
		messageOut = handleNotesMethod(parts[1], messageIn.Params)
	default:
		messageOut = serviceNotFoundError(parts[0])
	}

	data, err := json.Marshal(messageOut)
	return data, err
}
