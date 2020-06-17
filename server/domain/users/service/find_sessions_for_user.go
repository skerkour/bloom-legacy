package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) FindSessionsForUser(ctx context.Context, userID uuid.UUID) (ret []users.Session, err error) {
	return
}

/*

	var ret *SessionConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	sessions, err := users.FindAllSessionsForUserID(ctx, nil, *user.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

*/
