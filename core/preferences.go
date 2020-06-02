package core

import (
	"context"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/preferences"
	"gitlab.com/bloom42/bloom/core/messages"
)

func handlePreferencesMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "get":
		var params messages.PreferencesGetParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := preferences.Get(context.Background(), nil, params.Key)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "set":
		var params messages.PreferencesSetParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		err = preferences.Set(context.Background(), nil, params.Key, params.Value)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: messages.Empty{}}
	case "delete":
		var params messages.PreferencesDeleteParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		err = preferences.Delete(context.Background(), nil, params.Key)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: messages.Empty{}}
	default:
		return methodNotFoundError(method, "preferences")
	}
}
