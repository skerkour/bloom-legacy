package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
)

// Users finds all users
func (resolver *Resolver) Users(ctx context.Context) (*model.UserConnection, error) {
	var ret *model.UserConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	users, err := users.FindAllUsers(ctx)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &model.UserConnection{
		Nodes:      []*model.User{},
		TotalCount: int64(len(users)),
	}

	for _, user := range users {
		usr := &model.User{
			ID:          &user.ID,
			AvatarURL:   nil,
			CreatedAt:   &user.CreatedAt,
			Username:    user.Username,
			FirstName:   &user.FirstName,
			LastName:    &user.LastName,
			DisplayName: user.DisplayName,
			IsAdmin:     user.IsAdmin,
			Bio:         user.Bio,
			Email:       &user.Email,
			PublicKey:   user.PublicKey,
		}
		ret.Nodes = append(ret.Nodes, usr)
	}

	return ret, nil
}
