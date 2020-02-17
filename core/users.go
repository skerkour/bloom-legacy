package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/users"
)

func handleUsersMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "start_registration":
		var params users.StartRegistrationParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := users.StartRegistration(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "verify_registration":
		var params users.VerifyRegistrationParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := users.VerifyRegistration(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "complete_registration":
		var params users.CompleteRegistrationParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := users.CompleteRegistration(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "sign_in":
		var params users.SignInParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := users.SignIn(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "sign_out":
		err := users.SignOut()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: kernel.Empty{}}
	case "fetch_my_profile":
		res, err := users.FetchMyProfile()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "fetch_my_sessions":
		res, err := users.FetchMySessions()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "revoke_session":
		var params users.RevokeSessionParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		err = users.RevokeSession(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: kernel.Empty{}}
	case "update_profile":
		var input model.UserProfileInput
		err := json.Unmarshal(jsonParams, &input)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		user, err := users.UpdateProfile(input)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: user}
	default:
		return methodNotFoundError(method, "users")
	}
}
