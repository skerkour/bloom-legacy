package model

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/libs/rz-go"
)

type User struct {
	ID          *string    `json:"id"`
	CreatedAt   *time.Time `json:"createdAt"`
	Username    string     `json:"username"`
	FirstName   *string    `json:"firstName"`
	LastName    *string    `json:"lastName"`
	DisplayName string     `json:"displayName"`
	IsAdmin     bool       `json:"isAdmin"`
}

type UserResolver struct{}

func (resolver *UserResolver) GroupInvitations(ctx context.Context, user *User) ([]*GroupInvitation, error) {
	ret := &rpc.InvitationList{Invitations: []*rpc.Invitation{}}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return ret, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedUser == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return ret, twerr
	}

	invitations := []invit{}
	err := db.DB.Select(&invitations, `SELECT invit.id AS invitation_id, invit.created_at AS invitation_created_at,
	groups.id AS group_id, groups.created_at AS group_created_at, groups.name AS group_name, groups.description AS group_description,
		users.username AS inviter_username, users.display_name AS inviter_display_name
		FROM groups_invitations AS invit, groups, users
		WHERE invit.group_id = groups.id AND invit.invitee_id = $1 AND users.id = invit.inviter_id`, apiCtx.AuthenticatedUser.ID)
	if err != nil {
		logger.Error("groups.ListGroups: fetching invitations", rz.Err(err))
		return ret, twirp.InternalError("Internal error fetching invitations. Please try again.")
	}

	for _, invitation := range invitations {
		ret.Invitations = append(ret.Invitations, invitToRpcInvitation(invitation))
	}
	return ret, nil
}

func (resolver *UserResolver) Groups(ctx context.Context, user *User) ([]*Group, error) {
	var ret []*Group
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != *user.ID && !currentUser.IsAdmin {
		return ret, gqlerrors.New(errors.New(errors.PermissionDenied, "You have no right to access the sessions field"))
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
