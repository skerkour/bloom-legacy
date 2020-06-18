package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) RevokeSession(ctx context.Context, sessionID uuid.UUID) (err error) {
	me, err := service.Me(ctx)
	if err != nil {
		return
	}

	session, err := service.usersRepo.FindSessionByID(ctx, service.db, sessionID)
	if err != nil {
		return
	}

	if session.UserID != me.ID && !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	err = service.usersRepo.DeleteSession(ctx, service.db, sessionID)
	return
}
