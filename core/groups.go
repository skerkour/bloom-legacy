package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/domain/groups"
)
import "gitlab.com/bloom42/bloom/core/domain/kernel"

func handleGroupsMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "create_group":
		var params model.CreateGroupInput
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := groups.CreateGroup(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "delete_group":
		var params model.DeleteGroupInput
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		err = groups.DeleteGroup(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: kernel.Empty{}}
	default:
		return methodNotFoundError(method, "groups")
	}
}
