package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) DisableUser(ctx context.Context, userID uuid.UUID) (err error) {
	me, err := service.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	if !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}
	if me.ID == userID {
		err = errors.InvalidArgument("You cannot disable yourself")
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "users.DisableUser: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	user, err := service.usersRepo.FindUserByID(ctx, tx, userID)
	if err != nil {
		tx.Rollback()
		return
	}

	if user.DisabledAt != nil {
		err = errors.InvalidArgument("User is already disabled")
		tx.Rollback()
		return
	}

	now := time.Now().UTC()
	user.DisabledAt = &now
	user.UpdatedAt = now
	err = service.usersRepo.UpdateUser(ctx, tx, user)
	if err != nil {
		tx.Rollback()
		return
	}

	err = service.usersRepo.DeleteAllSessionsForUser(ctx, tx, user.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "users.DisableUser: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
