package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *GroupsRepository) CreateMembership(ctx context.Context, db db.Queryer, membership groups.Membership) (err error) {
	query := `INSERT INTO groups_members
	(joined_at, inviter_id, group_id, user_id, role, encrypted_master_key, master_key_nonce)
	VALUES ($1, $2, $3, $4, $5, $6, $7)`

	_, err = db.Exec(ctx, query, membership.JoinedAt, membership.InviterID, membership.GroupID,
		membership.UserID, membership.Role, membership.EncryptedMasterKey, membership.MasterKeyNonce)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.CreateMembership: inserting membership"
		logger.Error(errMessage, log.Err("error", err),
			log.UUID("group.id", membership.GroupID), log.UUID("user.id", membership.UserID))
		err = errors.Internal(errMessage, err)
	}
	return
}
