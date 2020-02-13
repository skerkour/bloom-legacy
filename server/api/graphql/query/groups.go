package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

func (r *Resolver) Groups(ctx context.Context) (*model.GroupConnection, error) {
	var ret *model.GroupConnection
	// ret := &rpc.GroupList{Groups: []*rpc.Group{}}
	// logger := rz.FromCtx(ctx)
	// apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	// if !ok {
	// 	return ret, twirp.InternalError("internal error")
	// }
	// if apiCtx.AuthenticatedUser == nil {
	// 	twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
	// 	return ret, twerr
	// }

	// if !apiCtx.AuthenticatedUser.IsAdmin {
	// 	return ret, twirp.NewError(twirp.PermissionDenied, "permission denied")
	// }

	// groups := []groups.Group{}
	// err := db.DB.Select(&groups, `SELECT * FROM groups`)
	// if err != nil {
	// 	logger.Error("groups.ListAllGroups: fetching groups", rz.Err(err))
	// 	return ret, twirp.InternalError("Internal error. Please try again.")
	// }

	// for _, group := range groups {
	// 	rpcGroup := rpc.Group{
	// 		Id:          group.ID,
	// 		CreatedAt:   group.CreatedAt.Format(time.RFC3339),
	// 		Name:        group.Name,
	// 		Description: group.Description,
	// 	}
	// 	ret.Groups = append(ret.Groups, &rpcGroup)
	// }
	return ret, nil
}
