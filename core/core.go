package core

import (
	"context"
	"encoding/json"
	"errors"
	"strings"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
	"gitlab.com/bloom42/bloom/core/domain/users"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
)

func Init(params InitParams) (InitRes, error) {
	var err error
	ret := InitRes{
		Preferences: map[string]interface{}{},
	}
	client := api.Client()

	log.Debug("Initializing core", rz.Any("params", params))
	if params.Env != "development" && params.Env != "dev" {
		log.SetLogger(log.With(rz.Level(rz.InfoLevel)))
	}

	kernel.Env = params.Env

	err = db.Init(params.DBKey)
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
		kernel.Me = signedIn.Me
	}

	ctx := context.Background()
	for _, key := range params.Preferences {
		value, err := preferences.Get(ctx, nil, key)
		if err == nil {
			ret.Preferences[key] = value
		}
	}

	// start background sync
	err = objects.Init()

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
	case "sync":
		err := objects.Sync(true)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: messages.Empty{}}
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
