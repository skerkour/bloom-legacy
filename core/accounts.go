package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/bloom/accounts"
	"gitlab.com/bloom42/bloom/core/bloom/kernel"
)

func handleAccountsMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "start_registration":
		var params accounts.StartRegistrationParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := accounts.StartRegistration(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "verify_registration":
		var params accounts.VerifyRegistrationParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := accounts.VerifyRegistration(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "complete_registration":
		var params accounts.CompleteRegistrationParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := accounts.CompleteRegistration(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "sign_in":
		var params accounts.SignInParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := accounts.SignIn(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "sign_out":
		err := accounts.SignOut()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: kernel.Empty{}}
	default:
		return methodNotFoundError(method, "accounts")
	}
}
