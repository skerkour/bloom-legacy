package handler

import (
	"context"
	"time"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/libs/rz-go"
)

func (handler Handler) ListGroups(ctx context.Context, _ *rpc.Empty) (*rpc.GroupList, error) {
	ret := &rpc.GroupList{Groups: []*rpc.Group{}}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return ret, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedUser == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return ret, twerr
	}

	groups := []groups.Group{}
	err := db.DB.Select(`SELECT * FROM groups
		INNER JOIN groups_members ON groups.id = groups_members.group_id
		WHERE groups_members.user_id = $1`, apiCtx.AuthenticatedUser.ID)
	if err != nil {
		logger.Error("users.ListGroups: fetching groups", rz.Err(err))
		return ret, twirp.InternalError("Internal error. Please try again.")
	}

	for _, group := range groups {
		rpcGroup := rpc.Group{
			Id:          group.ID,
			CreatedAt:   group.CreatedAt.Format(time.RFC3339),
			Name:        group.Name,
			Description: group.Description,
		}
		ret.Groups = append(ret.Groups, &rpcGroup)
	}

	return ret, nil
}
