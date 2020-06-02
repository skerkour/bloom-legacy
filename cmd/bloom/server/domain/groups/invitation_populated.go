package groups

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

type InvitationPopulated struct {
	ID                 uuid.UUID `db:"invitation_id"`
	CreatedAt          time.Time `db:"invitation_created_at"`
	GroupID            string    `db:"invitation_group_id"`
	InviterID          string    `db:"inviter_id"`
	InviterAvatarID    *string   `db:"inviter_avatar_id"`
	InviterUsername    string    `db:"inviter_username"`
	InviterDisplayName string    `db:"inviter_display_name"`
	InvitedID          string    `db:"invitee_id"`
	InviteeAvatarID    *string   `db:"invitee_avatar_id"`
	InviteeUsername    string    `db:"invitee_username"`
	InviteeDisplayName string    `db:"invitee_display_name"`
}
