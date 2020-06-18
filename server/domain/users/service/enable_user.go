package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) EnableUser(ctx context.Context, userID uuid.UUID) (err error) {
	me, err := service.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	if !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "users.EnableUser: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	user, err := service.usersRepo.FindUserByID(ctx, tx, userID)
	if err != nil {
		tx.Rollback()
		return
	}

	if user.DisabledAt == nil {
		err = errors.InvalidArgument("User is not disabled")
		tx.Rollback()
		return
	}

	now := time.Now().UTC()
	user.DisabledAt = nil
	user.UpdatedAt = now
	err = service.usersRepo.UpdateUser(ctx, tx, user)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "users.EnableUser: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
