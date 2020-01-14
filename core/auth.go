package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/bloom/auth"
	"gitlab.com/bloom42/bloom/core/bloom/kernel"
)

func HandleAuthMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "start_registration":
		var params auth.StartRegistrationParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return MethodNotFoundError(method, "auth") // TODO(z0mbie42): return error
		}
		res, err := auth.StartRegistration(params)
		if err != nil {
			return MethodNotFoundError(method, "auht") // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "verify_registration":
		var params auth.VerifyRegistrationParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return MethodNotFoundError(method, "auth") // TODO(z0mbie42): return error
		}
		res, err := auth.VerifyRegistration(params)
		if err != nil {
			return MethodNotFoundError(method, "auht") // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "complete_registration":
		var params auth.CompleteRegistrationParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return MethodNotFoundError(method, "auth") // TODO(z0mbie42): return error
		}
		res, err := auth.CompleteRegistration(params)
		if err != nil {
			return MethodNotFoundError(method, "auht") // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "sign_in":
		var params auth.SignInParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := auth.SignIn(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "sign_out":
		err := auth.SignOut()
		if err != nil {
			return MethodNotFoundError(method, "auht") // TODO(z0mbie42): return error
		}
		return MessageOut{Data: kernel.Empty{}}
	default:
		return MethodNotFoundError(method, "auht")
	}
}
