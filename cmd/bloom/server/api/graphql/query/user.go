package query

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/uuid"
)

// User finds an user
// username is optional for a future use
func (resolver *Resolver) User(ctx context.Context, username *string) (ret *model.User, err error) {
	currentUser := apiutil.UserFromCtx(ctx)
	var firstName *string
	var lastName *string
	var email *string
	var createdAt *time.Time
	var id *uuid.UUID
	isAdmin := false
	var disabledAt *time.Time

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	if username == nil {
		err = gqlerrors.Internal()
		return
	}

	user, err := users.FindUserByUsername(ctx, nil, *username)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	if user.ID == currentUser.ID || currentUser.IsAdmin {
		id = &user.ID
		createdAt = &user.CreatedAt
		firstName = &user.FirstName
		lastName = &user.LastName
		isAdmin = user.IsAdmin
		email = &user.Email
		disabledAt = user.DisabledAt
	}

	ret = &model.User{
		ID:          id,
		AvatarURL:   nil,
		CreatedAt:   createdAt,
		Username:    user.Username,
		FirstName:   firstName,
		LastName:    lastName,
		DisplayName: user.DisplayName,
		IsAdmin:     isAdmin,
		Bio:         user.Bio,
		Email:       email,
		DisabledAt:  disabledAt,
		PublicKey:   user.PublicKey,
	}
	return
}
