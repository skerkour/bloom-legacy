package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) FindSessionsForUser(ctx context.Context, userID uuid.UUID) (ret []users.Session, err error) {
	ret = []users.Session{}
	me, err := service.Me(ctx)
	if err != nil {
		return
	}

	if me.ID != user.ID && !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	ret, err = service.usersRepo.FindAllSessionsForUser(ctx, service.db, userID)
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
