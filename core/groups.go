package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/domain/groups"
	"gitlab.com/bloom42/bloom/core/messages"
)
import "context"

func handleGroupsMethod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "createGroup":
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
	case "deleteGroup":
		var params model.DeleteGroupInput
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		err = groups.DeleteGroup(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: messages.Empty{}}
	case "findGroups":
		res, err := groups.FindGroups(context.Background(), nil)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "fetchGroupMembers":
		var params messages.FetchGroupMembersParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := groups.FetchGroupMembers(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "inviteUsers":
		var params model.InviteUsersInGroupInput
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := groups.InviteUsers(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "removeMembers":
		var params model.RemoveGroupMembersInput
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := groups.RemoveMembers(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "fetchGroups":
		res, err := groups.FetchGroups()
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	case "fetchGroupDetails":
		var params messages.FetchGroupDetailsParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := groups.FetchGroupDetails(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return methodNotFoundError(method, "groups")
	}
}
