package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

func (repo *GroupsRepository) CreateMembership(ctx context.Context, db db.Queryer, membership groups.Membership) (err error) {
	return
}

/*
	// create membership
	queryCreateMembership := `INSERT INTO groups_members
		(joined_at, inviter_id, group_id, user_id, role, encrypted_master_key, master_key_nonce)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err = tx.Exec(queryCreateMembership, membership.JoinedAt, membership.InviterID, membership.GroupID,
		membership.UserID, membership.Role, membership.EncryptedMasterKey, membership.MasterKeyNonce)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.AcceptInvitation: creating membership", rz.Err(err))
		return ret, NewError(ErrorAcceptingInvitation)
	}

*/
