package core

import (
	"encoding/json"
	"errors"
	"strings"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/users"
)

func Init() (*model.SignedIn, error) {
	var ret *model.SignedIn
	client := api.Client()

	err := db.Init()
	if err != nil {
		return ret, err
	}

	ret, err = users.FindPersistedSession()
	if err != nil {
		return ret, nil
	}
	if ret != nil {
		client.Authenticate(ret.Session.ID, *ret.Session.Token)
	}

	return ret, err
}

func handleCoreMethod(method string, _ json.RawMessage) MessageOut {
	switch method {
	case "init":
		res, err := Init()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
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
	case "admin":
		messageOut = handleAdminMehtod(parts[1], messageIn.Params)
	case "billing":
		messageOut = handleBillingMehtod(parts[1], messageIn.Params)
	case "groups":
		messageOut = handleGroupsMethod(parts[1], messageIn.Params)
	default:
		messageOut = serviceNotFoundError(parts[0])
	}

	data, err := json.Marshal(messageOut)
	return data, err
}
