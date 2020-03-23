package core

import (
	"context"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
)

func handlePreferencesMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "get":
		var params preferences.GetParams
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
		var params preferences.SetParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		err = preferences.Set(context.Background(), nil, params.Key, params.Value)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: kernel.Empty{}}
	case "delete":
		var params preferences.DeleteParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		err = preferences.Delete(context.Background(), nil, params.Key)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: kernel.Empty{}}
	default:
		return methodNotFoundError(method, "preferences")
	}
}
