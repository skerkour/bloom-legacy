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
	return nil, nil
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
