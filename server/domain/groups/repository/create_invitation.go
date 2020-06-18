package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

func (repo *GroupsRepository) CreateInvitation(ctx context.Context, db db.Queryer, invitation groups.Invitation) (err error) {
	return
}

/*
	queryInsertInvitation := `INSERT INTO groups_invitations
		(id, created_at, updated_at, ephemeral_public_key, signature, encrypted_master_key,
			group_id, invitee_id, inviter_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`
	_, err = tx.Exec(queryInsertInvitation, invitation.ID, invitation.CreatedAt, invitation.UpdatedAt,
		invitation.EphemeralPublicKey, invitation.Signature, invitation.EncryptedMasterKey,
		invitation.GroupID, invitation.InviteeID, invitation.InviterID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUser: inserting new invitation", rz.Err(err))
		err = NewError(ErrorInvitingUsers)
		return
	}
*/
