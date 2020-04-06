package query

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/bloom/server/domain/users"
)

// User finds an user
func (resolver *Resolver) User(ctx context.Context, username *string) (*model.User, error) {
	var ret *model.User
	currentUser := apiutil.UserFromCtx(ctx)
	var firstName *string
	var lastName *string
	var email *string
	var createdAt *time.Time
	var id *string
	isAdmin := false
	var disabledAt *time.Time

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	if username == nil {
		return ret, gqlerrors.Internal()
	}

	user, err := users.FindUserByUsernameNoTx(ctx, *username)
	if err != nil {
		return ret, gqlerrors.New(err)
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
		PublicKey:   model.Bytes(user.PublicKey),
	}

	return ret, nil
}
