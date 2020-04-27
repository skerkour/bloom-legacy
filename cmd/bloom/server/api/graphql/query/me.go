package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

// Me returns the current user
func (resolver *Resolver) Me(ctx context.Context) (ret *model.User, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	ret = &model.User{
		ID:                  &currentUser.ID,
		AvatarURL:           nil,
		CreatedAt:           &currentUser.CreatedAt,
		Username:            currentUser.Username,
		FirstName:           &currentUser.FirstName,
		LastName:            &currentUser.LastName,
		DisplayName:         currentUser.DisplayName,
		IsAdmin:             currentUser.IsAdmin,
		Bio:                 currentUser.Bio,
		Email:               &currentUser.Email,
		EncryptedPrivateKey: &currentUser.EncryptedPrivateKey,
		PublicKey:           currentUser.PublicKey,
	}
	return ret, nil
}
