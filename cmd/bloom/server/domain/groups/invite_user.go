package groups

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

type InviteUserParams struct {
	GroupID                     uuid.UUID
	Username                    string
	EphemeralPublicKey          []byte
	InvitationSignature         []byte
	EncryptedMasterKey          []byte
	EncryptedMasterKeySignature []byte
}

func InviteUser(ctx context.Context, actor *users.User, params InviteUserParams) (retGroup *Group, err error) {
	logger := rz.FromCtx(ctx)
	var inviteeId uuid.UUID
	var group Group
	var invitation *Invitation

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.InviteUser: Starting transaction", rz.Err(err))
		err = NewError(ErrorInvitingUsers)
		return
	}

	queryGetGroup := "SELECT * FROM groups WHERE id = $1"
	err = tx.Get(&group, queryGetGroup, params.GroupID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUser: fetching group", rz.Err(err),
			rz.String("group.id", params.GroupID.String()))
		err = NewError(ErrorGroupNotFound)
		return
	}

	queryStr := "SELECT id FROM users WHERE username = $1"
	err = tx.Get(&inviteeId, queryStr, params.Username)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUser: error fetching invitee", rz.Err(err))
		err = NewError(ErrorInvitingUsers)
		return
	}

	if err = CheckUserIsGroupAdmin(ctx, tx, actor.ID, group.ID); err != nil {
		return
	}

	err = validateInviteUser(ctx, tx, actor, group, inviteeId)
	if err != nil {
		tx.Rollback()
		return
	}

	// create invitation
	now := time.Now().UTC()
	invitation = &Invitation{
		ID:                          uuid.New(),
		CreatedAt:                   now,
		UpdatedAt:                   now,
		EphemeralPublicKey:          params.EphemeralPublicKey,
		InvitationSignature:         params.InvitationSignature,
		EncryptedMasterKey:          params.EncryptedMasterKey,
		EncryptedMasterKeySignature: params.EncryptedMasterKeySignature,
		GroupID:                     group.ID,
		InviteeID:                   inviteeId,
		InviterID:                   actor.ID,
	}
	queryInsertInvitation := `INSERT INTO groups_invitations
		(id, created_at, updated_at, ephemeral_public_key, invitation_signature, encrypted_master_key,
			encrypted_master_key_signature, group_id, invitee_id, inviter_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`
	_, err = tx.Exec(queryInsertInvitation, invitation.ID, invitation.CreatedAt, invitation.UpdatedAt,
		invitation.EphemeralPublicKey, invitation.InvitationSignature, invitation.EncryptedMasterKey,
		invitation.EncryptedMasterKeySignature, invitation.GroupID, invitation.InviteeID, invitation.InviterID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUser: inserting new invitation", rz.Err(err))
		err = NewError(ErrorInvitingUsers)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUser: Committing transaction", rz.Err(err))
		err = NewError(ErrorInvitingUsers)
		return
	}

	retGroup = &group
	return
}

func validateInviteUser(ctx context.Context, tx *sqlx.Tx, inviter *users.User, group Group, inviteeID uuid.UUID) error {
	logger := rz.FromCtx(ctx)
	var alreadyInUsers int
	var err error

	// check that inviter inviting is admin
	if err = CheckUserIsGroupAdmin(ctx, tx, inviter.ID, group.ID); err != nil {
		return err
	}

	//  check that user is not already in group or awaiting invitations
	queryStr := `SELECT COUNT(*)
		FROM groups_members, groups_invitations
		WHERE (groups_members.group_id = $1 AND groups_members.user_id = $2)
			OR (groups_invitations.invitee_id = $2)`

	// query, args, err := sqlx.In(queryStr, group.ID, inviteesIds)
	// query = tx.Rebind(query)
	err = tx.Get(&alreadyInUsers, queryStr, group.ID, inviteeID)
	if err != nil {
		logger.Error("groups.InviteUser: error fetching user already in group or invitations", rz.Err(err))
		return NewError(ErrorInvitingUsers)
	}
	if alreadyInUsers != 0 {
		return NewError(ErrorUserAlreadyInGroup)
	}

	return nil
}
