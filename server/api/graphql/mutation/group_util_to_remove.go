package mutation

import (
	"time"
)

type invit struct {
	ID                 string    `db:"invitation_id"`
	CreatedAt          time.Time `db:"invitation_created_at"`
	GroupID            string    `db:"group_id"`
	GroupCreatedAt     time.Time `db:"group_created_at"`
	GroupName          string    `db:"group_name"`
	GroupDescription   string    `db:"group_description"`
	InviterUsername    string    `db:"inviter_username"`
	InviterDisplayName string    `db:"inviter_display_name"`
}

func invitToRpcInvitation(invitation invit) *rpc.Invitation {
	return &rpc.Invitation{
		Id: invitation.ID,
		Group: &rpc.Group{
			Id:          invitation.GroupID,
			CreatedAt:   invitation.GroupCreatedAt.Format(time.RFC3339),
			Name:        invitation.GroupName,
			Description: invitation.GroupDescription,
		},
		Inviter: &rpc.Inviter{
			Username:    invitation.InviterUsername,
			DisplayName: invitation.InviterDisplayName,
		},
	}
}
