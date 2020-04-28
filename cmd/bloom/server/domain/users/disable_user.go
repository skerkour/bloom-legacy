package users

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

// DisableUser is used to disable an user
func DisableUser(ctx context.Context, actor *User, userID uuid.UUID) error {
	logger := rz.FromCtx(ctx)

	if actor == nil || !actor.IsAdmin {
		return NewError(ErrorAdminRoleRequired)
	}

	if actor.ID == userID {
		return NewError(ErrorCantDisableYourself)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("Starting transaction", rz.Err(err))
		return NewError(ErrorInternal)
	}

	user, err := FindUserByID(ctx, tx, userID)
	if err != nil {
		tx.Rollback()
		return err
	}

	if user.DisabledAt == nil {
		tx.Rollback()
		return NewError(ErrorUserNotDisabled)
	}

	now := time.Now().UTC()
	user.DisabledAt = &now
	user.UpdatedAt = now

	_, err = tx.Exec("UPDATE users SET updated_at = $1, disabled_at = $2 WHERE id = $3",
		user.UpdatedAt, user.DisabledAt, user.ID)
	if err != nil {
		logger.Error("disabling user", rz.Err(err), rz.String("user.id", user.ID.String()))
		return NewError(ErrorInternal)
	}

	_, err = tx.Exec("DELETE FROM sessions WHERE user_id = $3", user.ID)
	if err != nil {
		logger.Error("deleting user sessions", rz.Err(err), rz.String("user.id", user.ID.String()))
		return NewError(ErrorInternal)
	}

	// commit db transaction
	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("Committing transaction", rz.Err(err))
		return NewError(ErrorInternal)
	}

	return nil
}
