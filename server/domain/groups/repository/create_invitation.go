package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *GroupsRepository) CreateInvitation(ctx context.Context, db db.Queryer, invitation groups.Invitation) (err error) {
	query := `INSERT INTO groups_invitations
	(id, created_at, updated_at, ephemeral_public_key, signature, encrypted_master_key,
		group_id, invitee_id, inviter_id)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`

	_, err = db.Exec(ctx, query, invitation.ID, invitation.CreatedAt, invitation.UpdatedAt,
		invitation.EphemeralPublicKey, invitation.Signature, invitation.EncryptedMasterKey,
		invitation.GroupID, invitation.InviteeID, invitation.InviterID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.CreateInvitation: inserting invitation"
		logger.Error(errMessage, log.Err("error", err), log.UUID("invitation.id", invitation.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
