package core

import (
	"context"
	"encoding/json"
	"errors"
	"strings"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
	"gitlab.com/bloom42/bloom/core/domain/users"
	"gitlab.com/bloom42/lily/rz/log"
	"gitlab.com/bloom42/lily/keyring"
)

func Init(params InitParams) (InitRes, error) {
	var dBKey string
	var err error
	ret := InitRes{
		Preferences: map[string]interface{}{},
	}
	client := api.Client()

	if params.DBKey == nil {
		// desktop
		log.Info("Fetching db_key from system's secret store")
		// fetch key, if not found, generate it
		dBKey, err = keyring.Get("com.bloom42.bloom", "db_key")
		if err != nil {
			dBKey = "TODO"
		}
	} else {
		dBKey = *params.DBKey
	}

	err = db.Init(dBKey)
	if err != nil {
		return ret, err
	}

	signedIn, err := users.FindPersistedSession()
	if err != nil {
		return ret, nil
	}
	if signedIn != nil {
		client.Authenticate(signedIn.Session.ID, *signedIn.Session.Token)
		ret.Preferences["me"] = signedIn.Me
		ret.Preferences["session"] = signedIn.Session
	}

	ctx := context.Background()
	for _, key := range params.Preferences {
		value, err := preferences.Get(ctx, nil, key)
		if err == nil {
			ret.Preferences[key] = value
		}
	}

	return ret, err
}

func handleCoreMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "init":
		var params InitParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := Init(params)
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
	case "preferences":
		messageOut = handlePreferencesMehtod(parts[1], messageIn.Params)
	default:
		messageOut = serviceNotFoundError(parts[0])
	}

	data, err := json.Marshal(messageOut)
	return data, err
}
