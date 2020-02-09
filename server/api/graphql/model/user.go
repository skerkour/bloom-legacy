package model

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/libs/rz-go"
)

type User struct {
	ID          *string    `json:"id"`
	AvatarURL   *string    `json:"avatarUrl"`
	CreatedAt   *time.Time `json:"createdAt"`
	Username    string     `json:"username"`
	FirstName   *string    `json:"firstName"`
	LastName    *string    `json:"lastName"`
	DisplayName string     `json:"displayName"`
	IsAdmin     bool       `json:"isAdmin"`
	Bio         string     `json:"bio"`
	Email       *string    `json:"email"`
}

type UserResolver struct{}

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

func (resolver *UserResolver) GroupInvitations(ctx context.Context, user *User) ([]*GroupInvitation, error) {
	var ret []*GroupInvitation
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	if currentUser.ID != *user.ID && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	ret = []*GroupInvitation{}
	invitations := []invit{}
	err := db.DB.Select(&invitations, `SELECT invit.id AS invitation_id, invit.created_at AS invitation_created_at,
	groups.id AS group_id, groups.created_at AS group_created_at, groups.name AS group_name, groups.description AS group_description,
		users.username AS inviter_username, users.display_name AS inviter_display_name
		FROM groups_invitations AS invit, groups, users
		WHERE invit.group_id = groups.id AND invit.invitee_id = $1 AND users.id = invit.inviter_id`, user.ID)
	if err != nil {
		logger.Error("groups.ListGroups: fetching invitations", rz.Err(err))
		return ret, gqlerrors.Internal()
	}

	for _, invitation := range invitations {
		invitatio := &GroupInvitation{
			ID: invitation.ID,
			Group: &Group{
				Name:        invitation.GroupName,
				Description: invitation.GroupDescription,
			},
		}
		ret = append(ret, invitatio)
	}
	return ret, nil
}

func (resolver *UserResolver) Groups(ctx context.Context, user *User) ([]*Group, error) {
	var ret []*Group
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	if currentUser.ID != *user.ID && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	ret = []*Group{}
	logger := rz.FromCtx(ctx)

	groups := []groups.Group{}
	err := db.DB.Select(&groups, `SELECT * FROM groups
	INNER JOIN groups_members ON groups.id = groups_members.group_id
	WHERE groups_members.user_id = $1`, currentUser.ID)
	if err != nil {
		logger.Error("User.groups: fetching groups", rz.Err(err))
		return ret, gqlerrors.Internal()
	}

	for _, group := range groups {
		grp := Group{
			ID:          &group.ID,
			CreatedAt:   &group.CreatedAt,
			Name:        group.Name,
			Description: group.Description,
			//	members: [GroupMember!]
			// invitations: [GroupInvitation!]
		}
		ret = append(ret, &grp)
	}
	return ret, nil
}

func (resolver *UserResolver) Invoices(ctx context.Context, user *User) ([]*Invoice, error) {
	return nil, nil
}

func (resolver *UserResolver) PaymentMethods(ctx context.Context, user *User) ([]*PaymentMethod, error) {
	return nil, nil
}

func (resolver *UserResolver) Sessions(ctx context.Context, user *User) ([]*Session, error) {
	var ret []*Session
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != *user.ID && !currentUser.IsAdmin {
		return ret, gqlerrors.New(errors.New(errors.PermissionDenied, "You have no right to access the sessions field"))
	}

	ret = []*Session{}
	logger := rz.FromCtx(ctx)

	sessions := []users.Session{}
	err := db.DB.Select(&sessions, "SELCT * FROM sessions WHERE user_id = $1", currentUser.ID)
	if err != nil {
		logger.Error("User.sessions: fetching sessions", rz.Err(err))
		return ret, gqlerrors.Internal()
	}

	for _, session := range sessions {
		sess := Session{
			ID:        session.ID,
			CreatedAt: session.CreatedAt,
			Token:     nil,
			Device:    nil,
		}
		ret = append(ret, &sess)
	}

	return ret, nil
}

func (resolver *UserResolver) StripePublicKey(ctx context.Context, user *User) (*string, error) {
	var ret *string
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != *user.ID && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	ret = &config.Stripe.PublicKey
	return ret, nil
}

func (resolver *UserResolver) BillingPlan(ctx context.Context, user *User) (*BillingPlan, error) {
	var ret *BillingPlan
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != *user.ID && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	panic("not implemented")
}
