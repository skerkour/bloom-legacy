package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindGroupMasterKey(ctx context.Context, tx *sqlx.Tx, groupId uuid.UUID, userID uuid.UUID) (Membership, error) {
	ret := Membership{}
	var err error
	logger := rz.FromCtx(ctx)

	query := `SELECT encrypted_master_key, master_key_nonce FROM groups_members
		WHERE group_id = $1 AND user_id = $2`
	if tx == nil {
		err = db.DB.Get(&ret, query, groupId, userID)
	} else {
		err = tx.Get(&ret, query, groupId, userID)
	}
	if err != nil {
		logger.Error("finding group masterKey", rz.Err(err),
			rz.String("group.id", groupId.String()), rz.String("user.id", userID.String()))
		return ret, NewError(ErrorGroupNotFound)
	}

	return ret, err
}
